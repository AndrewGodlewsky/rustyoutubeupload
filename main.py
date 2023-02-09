import os

import clip


def main():
    os.makedirs("Input", exist_ok=True)

    videos = os.listdir("Input/")

    os.makedirs("Clips", exist_ok=True)
    os.makedirs("Music", exist_ok=True)

    for video in videos:
        if video.endswith(".mp4"):
            print(f"Clipping {video}")
            clip.start(f"Input/{video}")


if __name__ == "__main__":
    main()
