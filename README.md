# ScreenMacro

A tool to create and execute macro programs.

Release version exists for Windows, however all libraries *should* be compatible with other platforms as well, so compile at your own risk.

## Advantages

ScreenMacro provides commands to search and click on images provided by the user (thus, the "Screen" part of ScreenMacro), either as a file, or from the clipboard with snipping tool of your choice (tested only with Windows builtin snipping tool avaliable through Shift+Win+S).
ScreenMacro also supplies other commands you might need in your macro programs.

## Commands

### Launch

Runs a specified file. It can be anything, as long as it's an executable or a file with a format that it associated with an executable (e.g. an mp3 file would be run with your default music player).

### Click image

Takes in an image file or an image from the clipboard, that will be searched for and (left mouse button) clicked.

NOTE: since we need accurate images for scanning the desktop - only PNG files are allowed. Don't convert JPG to PNG and use it with the program - the quality was already lost. Using snipping tools and pasting from clipboard is safe, as it provides raw RGBA data.

Another note: images appear blurrier in the program, possibly because of iced image processing inaccuracy, test the command to check if the images match.

The rectangle with a dot is the point to be clicked on - avaliable positions is a 3x3 grid, covering 4 corners, centers of 4 sides and the center of the image (default).

Last input for the command is accuracy percentage - BE WARNED that lowering this won't help if your image and button you want to be clicked had a small amount of vastly different pixels. Once a pixel is found that is more different that the specified coefficient - the whole comparison fails. Can be useful, however, if the image and the button have any amount of almost similar pixels, like a light moving gradient.

### Move to image

Same as Click image, except, as the name suggests, instead of clicking the image, the command only moves the cursor to the image.

### Type text

Types the text specified at maximum speed possible.

Also you can specify to use modifiers of shift, alt or ctrl during typing.

### Press key

Presses any of the avaliable keys, that cannot be provided with the type text command.

Modifiers can be used, same as in type text.

### Scroll

Scrolls the mouse wheel up or down for the amount of "clicks" specified at the current position of the mouse. Clicks are roughly equvalent to normal mouse "snaps" of the mouse wheel, but I can't confim that all mice have the same "scroll pixels to snap ratio" so experiment to get what you need.

### Wait time

Waits for the time specified in milliseconds.

## Other notes

### Executing commands

There are play buttons for each command separately, which tests the commands. Commands that can take a long time (only images in this case) will timeout after 10 seconds, but will run completely at least once. The play button for the whole macro file minimizes the window (when iced 0.5 launches) and executes commands in the order as they were defined. If any long command times out (timeout is specified in settings) - macro stops either the command or the whole macro, as per defined in settings. Play button becomes a stop button while executing macros, so you can end the execution abruptly.

### Settings

Allows you to specifiy the timeout in seconds for long running commands and the behavior of timeout reaction. If "Stop whole macro on timeout" if on (default) then macro execution is stopped completely, otherwise the specific command is skipped and the macro continues normaly.

### Saving

When you modify your macro a "*" will appear near the name of the macro, indicating unsaved changes. Only "Save as" button is avaliable as of right now, so you can't save to the same file quickly, but you can override previous files as needed. The resulting files have ".smbf" format (which stands for ScreenMacro binary file or ScreenMacro binary format), created using serde, ciborium, and a custom serializable structure.