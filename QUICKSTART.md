# VoxWeave - Quick Start Guide

## 🚀 Get Started in 30 Seconds

### 1. Build
```bash
cargo build
```

### 2. Run Demo
```bash
./run_demo.sh
```

### 3. Verify Everything Works
```bash
./verify.sh
```

---

## 📖 Common Commands

### List All Voices
```bash
./target/debug/voxweave list-voices
```

### Convert Text (Mock Mode)
```bash
./target/debug/voxweave convert samples/example.txt --mock --voice en-us
```

### Convert with Custom Settings
```bash
./target/debug/voxweave convert input.txt \
  --voice en-us \
  --speed 1.2 \
  --subtitles sentence \
  --output ./output \
  --mock
```

### Get Help
```bash
./target/debug/voxweave --help
./target/debug/voxweave convert --help
```

---

## 🎯 Quick Test

Run this to test the project immediately:
```bash
cargo build && \
./target/debug/voxweave convert samples/example.txt --mock --voice en-us && \
echo "✅ VoxWeave is working!"
```

---

## 📂 Output Files

After conversion, you'll find:
- `<name>.wav` - Audio file (or text in mock mode)
- `<name>.srt` - Subtitle file (if enabled)

---

## 🔥 Pro Tips

1. **Use mock mode for testing** - No need to install espeak-ng
2. **Check verify.sh** - Runs all tests automatically
3. **Use --help** - See all available options
4. **Try different voices** - 68 voices available!

---

## ⚡ One-Liner Commands

**Full build + test:**
```bash
cargo clean && cargo build && cargo test && ./verify.sh
```

**Quick demo:**
```bash
./run_demo.sh
```

**Test conversion:**
```bash
./target/debug/voxweave convert samples/example.txt --mock --voice en-us
```

---

## ✅ Status Check

Everything working if you see:
- ✅ Build completes without errors
- ✅ All 17 tests pass
- ✅ Voices list displays 68+ voices
- ✅ Conversion creates .wav and .srt files

Run `./verify.sh` to check everything at once!

---

**Need help?** Check `FIXES_APPLIED.md` and `EXECUTION_STATUS.md` for details.
# VoxWeave - Quick Start Guide

## 🚀 Get Started in 30 Seconds

### 1. Build
```bash
cargo build
```

### 2. Run Demo
```bash
./run_demo.sh
```

### 3. Verify Everything Works
```bash
./verify.sh
```

---

## 📖 Common Commands

### List All Voices
```bash
./target/debug/voxweave list-voices
```

### Convert Text (Mock Mode)
```bash
./target/debug/voxweave convert samples/example.txt --mock --voice en-us
```

### Convert with Custom Settings
```bash
./target/debug/voxweave convert input.txt \
  --voice en-us \
  --speed 1.2 \
  --subtitles sentence \
  --output ./output \
  --mock
```

### Get Help
```bash
./target/debug/voxweave --help
./target/debug/voxweave convert --help
```

---

## 🎯 Quick Test

Run this to test the project immediately:
```bash
cargo build && \
./target/debug/voxweave convert samples/example.txt --mock --voice en-us && \
echo "✅ VoxWeave is working!"
```

---

## 📂 Output Files

After conversion, you'll find:
- `<name>.wav` - Audio file (or text in mock mode)
- `<name>.srt` - Subtitle file (if enabled)

---

## 🔥 Pro Tips

1. **Use mock mode for testing** - No need to install espeak-ng
2. **Check verify.sh** - Runs all tests automatically
3. **Use --help** - See all available options
4. **Try different voices** - 68 voices available!

---

## ⚡ One-Liner Commands

**Full build + test:**
```bash
cargo clean && cargo build && cargo test && ./verify.sh
```

**Quick demo:**
```bash
./run_demo.sh
```

**Test conversion:**
```bash
./target/debug/voxweave convert samples/example.txt --mock --voice en-us
```

---

## ✅ Status Check

Everything working if you see:
- ✅ Build completes without errors
- ✅ All 17 tests pass
- ✅ Voices list displays 68+ voices
- ✅ Conversion creates .wav and .srt files

Run `./verify.sh` to check everything at once!

---

**Need help?** Check `FIXES_APPLIED.md` and `EXECUTION_STATUS.md` for details.
