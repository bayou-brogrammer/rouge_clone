# Basic Roguelike
This is less of a tutorial, and more of a journal. Just wanted to make a super basic roguelike I can build off of and tinker with later on using the libraries I enjoy using. It's probably neither the best code, nor the best design for making one.

## How to Write a Roguelike in 15 Steps
https://roguebasin.com/index.php/How_to_Write_a_Roguelike_in_15_Steps
I intend on following these steps in each git branch.

## Step 5 - Saving/Loading
```
(First: Note that you can initially do away with save and load entirely - a feature not present in many early implementations - and generate your dungeons instead of hard-coding them, which is arguably a more roguelike approach!)

Read the map from a file instead of hard-coding it. You can play with it and test different configurations if you make it human-readable from the start.

Add the 'save' command and the procedures to save your game -- the map at first, then, gradually, all of other game elements. From now on, when you implement something that needs saving, also implement saving it as soon as possible.

Here's a handy article about save files and how to implement them; it's worthwhile to read when planning ahead what form you'll save things in.

Once you work with files, you can make your preferences and keybindings config files. Again, if you make them human-readable, you'll save yourself a lot of trouble.

Now, when you're not sure how any element of the game works, you can save the game to a file and just check it.
```
We'll be using bevy's reflect api. For now, I think making sure everything is reflectable and registering it on startup is fine.

We can work on some of the other things.