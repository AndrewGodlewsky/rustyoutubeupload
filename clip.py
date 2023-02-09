import os
import time
import random
from datetime import datetime
import json

from moviepy.editor import VideoFileClip, AudioFileClip


class TimeSpan:
    def __init__(self, start, end):
        self.startTime = start
        self.endTime = end

    def Duration(self) -> int:
        return self.endTime - self.startTime

    def Sum(timespans) -> int:
        return sum([t.Duration() for t in timespans])


class Video(object):
    def __init__(self, videoPath):
        self.videoFile: VideoFileClip = VideoFileClip(videoPath)
        self.timespans: list[TimeSpan] = []
        self.logging = None

        self.jsondata: list[dict] = []

    def generateContent(self):

        noun = random.choice(["J", "B"])

        title = random.choice([f"Hello {noun}", f"By {noun}"])

        print()



    def generateTimeSpans(self, possibleDurations: list[int]):
        while TimeSpan.Sum(self.timespans) < self.videoFile.duration:
            self.timespans.append(
                TimeSpan(
                    TimeSpan.Sum(self.timespans),
                    TimeSpan.Sum(self.timespans) + random.choice(possibleDurations),
                )
            )

        # Timespans must be less then the total length of the video
        if TimeSpan.Sum(self.timespans) > self.videoFile.duration:
            self.timespans.pop()
        return self

    def describe(self):
        print(f"Video Filename: {self.videoFile.filename}")
        print(f"Video Duration: {self.videoFile.duration}")
        print(f"Video FPS: {self.videoFile.fps}")

        print(f"\nVideo Timespans:")
        for timespan in self.timespans:
            print(
                f"Start: {timespan.startTime} ; "
                + f"End: {timespan.endTime} ; "
                + f"Duration: {timespan.Duration()} seconds"
            )

        return self

    def setAudio(
        self, videoClip: VideoFileClip, audioFile: AudioFileClip
    ) -> VideoFileClip:
        # A window at which the audio can start. 0 to x seconds
        audio_window = audioFile.duration - videoClip.duration

        audio_start_time = random.random() * audio_window
        audio_end_time = audio_start_time + videoClip.duration

        audio_clip = audioFile.subclip(audio_start_time, audio_end_time)
        return videoClip.set_audio(audio_clip)

    def process(self, audioFile: AudioFileClip, timespan: TimeSpan, index):
        videoClip: VideoFileClip = self.videoFile.subclip(
            timespan.startTime, timespan.endTime
        )
        videoAudioClip: VideoFileClip = self.setAudio(videoClip, audioFile)

        timestamp = datetime.now().strftime("%d_%m_%Y_%H_%M_%S")
        video_filename = f"{timespan.Duration()}s_{timestamp}_{index + 1}.mp4"
        audio_filename = f"{timespan.Duration()}s_{timestamp}_{index + 1}.mp3"

        os.makedirs("Temp", exist_ok=True)
        videoAudioClip.write_videofile(
            f"Clips/{video_filename}",
            temp_audiofile=f"Temp/{audio_filename}",
            logger=self.logging,
            # threads=os.cpu_count(),
        )
        os.removedirs("Temp")
        

        # Append data about the video to a "jsondata" list 
        data = {}
        
        data["path"] = video_filename
        data["title"] = "TEST TITLE"

    
        data["description"] = "Minecraft"
        data["music_path"] = audioFile.filename
        data["duration"] = timespan.Duration()

        data["tags"] = ["Minecraft", "MC", "Mods"]
        data["madeforkids"] = False

        data["original_filename"] = videoAudioClip.filename

        self.jsondata.append(data)


    def use_logging(self):
        self.logging = "bar"
        return self

    def clip(self):
        for index, timespan in enumerate(self.timespans):
            if len(os.listdir("Music/")) == 0:
                print("No Music in the `Music` folder")
                break

            music = random.choice(os.listdir("Music/"))
            audio = AudioFileClip(f"Music/{music}")
            self.process(audio, timespan, index)

        # Using the "jsondata" list, create a json file. 
        with open("Clips/data.json", "w") as jsonfile:
            json.dump(self.jsondata, jsonfile)

        return self


def start(videoPath: str):
    (
        Video(videoPath)
        .generateTimeSpans([10, 20, 30])
        .describe()
        #.use_logging()
        .clip()
    )
