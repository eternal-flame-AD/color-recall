# Color-Recall

A simple color recall game made with Rust and built as WebAssembly. Designed to test your recall precision of colors.

## How to play

1. You will be shown a color, you can look at it for as long as you want.

2. The color will disappear for 8 seconds.

3. You will be shown a color picker, you have to pick the color you saw. You can also take as long as you want because you are likely to do worse by taking more time.

4. The score is the CIEDE2000 ΔE’ between the color you picked and the original color. From social media it seems 5-8 is the average score for most people. Some people can do almost 2.0.

## Additional Features

- 6 color spaces that can be switched between on the fly, ranging from the most common to the most technical (sRGB, HSV, HSL, CIELAB, CIEXYZ, CIELCH).
  Hint: sRGB is most common, HSV is usually what you see on color pickers, LAB is the most technical, XYZ is just hard mode, LCH should be the easiest to use if you know how to use it.
- Colors with extreme brightness or low saturation are not tested due to low accuracy and high dependency on the display used. This is to ensure that the game is fair for everyone.
  If you choose a color outside of this range, a warning will be shown suggesting you to pick a different color.

## Share your score

We recommend #yume-color-recall on any social media platform.
