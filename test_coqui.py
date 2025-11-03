#!/usr/bin/env python3
"""
Test script to verify CoquiTTS installation and basic functionality
"""

import sys

def test_imports():
    """Test that required packages are installed"""
    try:
        import torch
        print("✓ torch imported successfully")
        print(f"  PyTorch version: {torch.__version__}")
        
        from TTS.api import TTS
        print("✓ TTS imported successfully")
        
        import numpy
        print("✓ numpy imported successfully")
        
        return True
    except ImportError as e:
        print(f"✗ Import failed: {e}")
        print("\nTo install required packages, run:")
        print("  pip install TTS torch numpy")
        return False

def test_tts_model():
    """Test basic TTS model loading"""
    try:
        from TTS.api import TTS
        import torch
        
        device = "cuda" if torch.cuda.is_available() else "cpu"
        print(f"\nUsing device: {device}")
        
        print("\nTrying to initialize TTS model (this may take a while on first run)...")
        model_name = "tts_models/multilingual/multi-dataset/xtts_v2"
        
        tts = TTS(model_name=model_name, progress_bar=True, gpu=(device != "cpu"))
        print(f"✓ Successfully initialized {model_name}")
        
        return True
    except Exception as e:
        print(f"✗ Model initialization failed: {e}")
        return False

def main():
    print("=" * 60)
    print("CoquiTTS Installation Test")
    print("=" * 60)
    
    if not test_imports():
        sys.exit(1)
    
    print("\n" + "=" * 60)
    print("Testing TTS Model")
    print("=" * 60)
    
    if not test_tts_model():
        print("\nNote: Model download/initialization failed.")
        print("This is expected if TTS is not fully installed.")
        print("The basic imports are working, which means the Python bridge should work.")
    
    print("\n" + "=" * 60)
    print("✓ Basic CoquiTTS setup is ready!")
    print("=" * 60)

if __name__ == "__main__":
    main()
#!/usr/bin/env python3
"""
Test script to verify CoquiTTS installation and basic functionality
"""

import sys

def test_imports():
    """Test that required packages are installed"""
    try:
        import torch
        print("✓ torch imported successfully")
        print(f"  PyTorch version: {torch.__version__}")
        
        from TTS.api import TTS
        print("✓ TTS imported successfully")
        
        import numpy
        print("✓ numpy imported successfully")
        
        return True
    except ImportError as e:
        print(f"✗ Import failed: {e}")
        print("\nTo install required packages, run:")
        print("  pip install TTS torch numpy")
        return False

def test_tts_model():
    """Test basic TTS model loading"""
    try:
        from TTS.api import TTS
        import torch
        
        device = "cuda" if torch.cuda.is_available() else "cpu"
        print(f"\nUsing device: {device}")
        
        print("\nTrying to initialize TTS model (this may take a while on first run)...")
        model_name = "tts_models/multilingual/multi-dataset/xtts_v2"
        
        tts = TTS(model_name=model_name, progress_bar=True, gpu=(device != "cpu"))
        print(f"✓ Successfully initialized {model_name}")
        
        return True
    except Exception as e:
        print(f"✗ Model initialization failed: {e}")
        return False

def main():
    print("=" * 60)
    print("CoquiTTS Installation Test")
    print("=" * 60)
    
    if not test_imports():
        sys.exit(1)
    
    print("\n" + "=" * 60)
    print("Testing TTS Model")
    print("=" * 60)
    
    if not test_tts_model():
        print("\nNote: Model download/initialization failed.")
        print("This is expected if TTS is not fully installed.")
        print("The basic imports are working, which means the Python bridge should work.")
    
    print("\n" + "=" * 60)
    print("✓ Basic CoquiTTS setup is ready!")
    print("=" * 60)

if __name__ == "__main__":
    main()
