## Images:

To encode images, run the program with the `i` parameter  
Then, specify the input file (MUST BE PNG!!!!)  
Then, specify the centiseconds when the subs should start (1/100th of a second)  
Then, specify the centiseconds when they end  
Then specify the mode (f = outputs ass file with headers, p = outputs just dialog lines)  
Then specify the target file

Example usage: `video2sub i input.png 0 500 f output.ass`

## Videos:

For videos, you need a video that's either 12.5 FPS, 25 FPS, 50 FPS or 100 FPS  
You need to split the video into individual frames USING THIS METHOD:

`ffmpeg -i thing.mp4 <folder>/%09d.png` (The format is important)

Then, run the program like so

`video2sub v <the directory you chose earlier> <number> <amount of frames> <output file>`

The directory you chose earlier is where ffmpeg dropped the frames

The numer specifies the framerate (12.5 = 1, 25 = 2, 50 = 4, 100 = 8)

Amount of frames... is amount of frames


Example:

```
ffmpeg -i video.mp4 folder/%09d.png
video2sub v folder 2 3000 out.ass
```

The naming of the frame files MUST be 9 digits + .png.