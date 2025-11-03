# 🚀 Feature: Z.AI Video Generation

## Descrição

Extensão do pipeline do VoxWeave que permite gerar vídeos automáticos com narração TTS, trilha sonora e visual IA, usando a API do Z.AI.

Esta feature combina o áudio TTS gerado localmente com visual IA gerado remotamente, sincronizando tudo no pipeline final (usando timestamps das legendas e MLT para composição).

## Funcionalidades

- ✅ Geração de vídeo automática com IA visual realista
- ✅ Sincronização com voz e legendas existentes  
- ✅ Configuração mínima (voz, estilo, resolução)
- ✅ Suporte cross-platform Rust (via reqwest + tokio)
- ✅ Integração total com pipeline e UI cyberpunk Dioxus
- ✅ Composição final usando MLT para merge de áudio e subtítulos word-by-word destacados

## Estilos Disponíveis

1. **BioTech Futurista** (`biotech`)
   - Partículas de DNA neon verde e azul
   - Camadas de dados transparentes
   - Iluminação suave de laboratório
   - Efeitos holográficos

2. **Cyberpunk Neon** (`cyberpunk`)
   - Estética neon magenta e cyan
   - Paisagens urbanas futuristas
   - Efeitos de chuva digital
   - Animações glitch nas legendas

3. **Educacional Limpo** (`educational`)
   - Visual limpo e minimalista
   - Tipografia clara
   - Esquemas de cores profissionais
   - Sincronização suave de legendas

4. **Realistic** - Cinematografia realista
5. **Anime** - Estilo anime vibrante
6. **3D** - Visuals 3D renderizados
7. **Cinematic** - Estética cinematográfica dramática

## Como Usar

### Via UI

1. Ative "🎬 Generate Video with Z.AI" checkbox
2. Selecione o estilo desejado (BioTech, Cyberpunk, Educational, etc.)
3. Escolha a resolução (720p, 1080p, 4K)
4. Configure voz e legendas normalmente
5. Clique em START

### Via Cargo Feature

```bash
cargo run --features "real-tts video-generation zai-video"
```

### Variáveis de Ambiente

```bash
export ZAI_API_KEY="sua-chave-api-zai"
```

## Arquitetura

### Fluxo de Processamento

1. **Geração de Áudio**: TTS local com Kokoro
2. **Geração de Legendas**: SRT/ASS/VTT local
3. **Geração de Vídeo Z.AI**:
   - Envia texto, voz e estilo para API Z.AI
   - Poll para status de geração
   - Download do vídeo gerado
4. **Composição Final MLT**:
   - Merge do vídeo Z.AI com áudio local
   - Adiciona subtítulos word-by-word destacados
   - Renderiza vídeo final com MLT XML

### Módulos

- **`crates/ui/services/zai_video.rs`**: Serviço principal de geração Z.AI
- **`crates/ui/services/mlt_video.rs`**: Composição final com MLT
- **`crates/ui/services/tts_service.rs`**: Integração no pipeline

## Output

- Vídeo temporário: `./output/zai_video.mp4`
- Vídeo final: `./output/zai_video_final.mp4` (com áudio e legendas sincronizados)

## API Z.AI

### Endpoint

```
POST https://api.z.ai/v1/video/generate
Authorization: Bearer {ZAI_API_KEY}
```

### Request Body

```json
{
  "text": "Texto a ser narrado...",
  "voice": "af_alloy",
  "style": "biotech",
  "subtitles": true,
  "resolution": "1080p",
  "prompt": "Prompt customizado opcional..."
}
```

### Response

```json
{
  "video_url": "https://...",
  "status": "processing",
  "job_id": "job-123"
}
```

### Status Polling

```
GET https://api.z.ai/v1/video/status/{job_id}
```

Retorna progresso e URL final do vídeo.

## Status de Progresso

- 10%: Criando job de geração
- 30%: Polling para conclusão
- 80%: Download do vídeo
- 90%: Composição final com MLT
- 100%: Completo

## Logs

A UI mostra logs em tempo real:
- "Starting Z.AI video generation with style: biotech"
- "Video generation job created"
- "Polling for video status..."
- "Video downloaded"
- "Composing final video with audio and subtitles using MLT..."
- "Final video composed: zai_video_final.mp4"

## Fallback

Se a geração Z.AI falhar, o sistema automaticamente:
1. Faz fallback para geração de vídeo padrão (se disponível)
2. Ou continua apenas com áudio e legendas
3. Loga mensagem de erro apropriada

## Requisitos

- `ZAI_API_KEY` definida no ambiente
- Feature `zai-video` habilitada
- Feature `video-generation` habilitada (dependência)
- Feature `real-tts` habilitada (para geração de áudio)
- MLT e FFmpeg instalados (para composição final)

---

**Versão**: 1.0.0  
**Status**: ✅ Implementado e funcional

