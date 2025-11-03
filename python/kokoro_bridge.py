import json
import sys
import wave

SAMPLE_RATE_FALLBACK = 24000


def _write_silence(path, sample_rate):
    with wave.open(path, "wb") as wav:
        wav.setnchannels(1)
        wav.setsampwidth(2)
        wav.setframerate(sample_rate)
        wav.writeframes(b"")


def _coerce_numpy(audio, np):
    # Handle torch tensors or numpy arrays gracefully
    if hasattr(audio, "detach"):
        audio = audio.detach()
    if hasattr(audio, "cpu"):
        audio = audio.cpu()
    if hasattr(audio, "numpy"):
        audio = audio.numpy()
    return np.asarray(audio, dtype="float32")


def main():
    payload = json.load(sys.stdin)
    text = payload.get("text", "")
    output_path = payload["output"]
    sample_rate = int(payload.get("sample_rate") or SAMPLE_RATE_FALLBACK)

    if not text.strip():
        _write_silence(output_path, sample_rate)
        sys.stdout.write(json.dumps({"duration": 0.0}))
        sys.stdout.flush()
        return

    try:
        from kokoro import KPipeline, model as kokoro_model
        import numpy as np
    except Exception as exc:
        sys.stderr.write(f"failed to import kokoro dependencies: {exc}\n")
        sys.stderr.flush()
        sys.exit(1)

    repo_id = payload.get("repo_id")
    device = payload.get("device") or "cpu"
    lang_code = payload.get("lang_code")
    voice = payload.get("voice")
    speed = float(payload.get("speed", 1.0))
    split_pattern = payload.get("split_pattern")

    if repo_id:
        kokoro_model.KModel.REPO_ID = repo_id

    pipeline = KPipeline(lang_code=lang_code, device=device or None)
    kwargs = {"voice": voice, "speed": speed}
    if split_pattern:
        kwargs["split_pattern"] = split_pattern

    segments = []
    for result in pipeline(text, **kwargs):
        audio = getattr(result, "audio", None)
        if audio is None:
            continue
        segments.append(_coerce_numpy(audio, np))

    if segments:
        audio = np.concatenate(segments).astype("float32")
    else:
        audio = np.zeros(0, dtype="float32")

    audio = np.clip(audio, -1.0, 1.0)
    pcm = (audio * 32767.0).astype("<i2")

    with wave.open(output_path, "wb") as wav:
        wav.setnchannels(1)
        wav.setsampwidth(2)
        wav.setframerate(sample_rate)
        wav.writeframes(pcm.tobytes())

    duration = float(audio.shape[0]) / float(sample_rate) if sample_rate else 0.0
    sys.stdout.write(json.dumps({"duration": duration}))
    sys.stdout.flush()


if __name__ == "__main__":
    main()
