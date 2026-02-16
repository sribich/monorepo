{
  pkgs,
  self,
  system,
}:
{
  default = pkgs.mkShell rec {
    buildInputs = with pkgs; [
      ffmpeg

      python312
      uv

      stdenv.cc
      stdenv.cc.cc.lib

      cudatoolkit
      cudaPackages.cudnn
      linuxPackages.nvidia_x11
    ];

    CUDA_PATH = pkgs.cudaPackages.cudatoolkit;
    CUDA_HOME = pkgs.cudaPackages.cudatoolkit;
    EXTRA_CCFLAGS = "-I/usr/include";
    EXTRA_LDFLAGS = "-L/lib -L${pkgs.linuxPackages.nvidia_x11}/lib";
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

    shellHook = ''
      echo "Entering WhisperX development shell with CUDA support"
      echo "Note: PyTorch and WhisperX will be installed via uv within a virtual environment"

      export LC_ALL="en_US.UTF-8"
      export LANG="en_US.UTF-8"
      export PYTHONIOENCODING="utf-8"

      if [[ ! -d ".venv" ]]; then
          echo "Creating python virtual environment"
          uv venv -p3.12
      fi

      # echo "Activating python virtual environment"
      source .venv/bin/activate

      # Set CUDA variables
      # export CUDA_VISIBLE_DEVICES=0
      # export XDG_CACHE_HOME="$HOME/.cache"

      # Install torch torchaudio for CUDA 12.1/12.2 (trying general cu12x)
      # echo "Installing latest stable torch and torchaudio for CUDA 12.x..."
      uv pip install torch torchaudio --index-url https://download.pytorch.org/whl/cu128
      # If cu121 still resolves to 12.6, we might try cu122 or no specific version
      # If problems persist, consider explicit torch versions that are known to work with cuDNN 8.x and CUDA 12.x
      # Example: pip install torch==2.1.2 torchaudio==2.1.2 --index-url https://download.pytorch.org/whl/cu121

      python3 -c "import torch; print('CUDA available' if torch.cuda.is_available() else 'CPU only')"

      # Install whisperx
      uv pip install -U whisperx

      echo "You are now using a NIX environment"
    '';
  };
}
