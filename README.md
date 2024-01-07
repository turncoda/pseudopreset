# pseudopreset

[PseudoMenu](https://www.nexusmods.com/pseudoregalia/mods/36) is a mod for
Pseudoregalia which allows custom map creators to specify a title, author,
level name, spawn point, and upgrades for their custom maps, in what is called
a "game preset". The manual way to do this is to import the included
F_GamePreset struct into Unreal Engine and use it to create a data table, then
include that in the level pak. This requires mappers to have Unreal Engine
installed and set up, which is not always convenient. For instance, it is
possible to create custom maps without Unreal Engine, and it would be a drag to
have to install Unreal Engine just to generate a game preset for your level.

That's where this tool comes in. With this tool, you can generate the game
preset data table without Unreal Engine. It works by modifying an existing
cooked data table which was created in Unreal Engine using the F_GamePreset
struct from PseudoMenu.

So say you have a level called MyLevel and you put it in `/Game/Mods/Maps`. In
your level you have a PlayerStart tagged 'myStart'. And you'd like the player
to start with the Dream Breaker, Slide, and Sun Greaves. Here's an example of
how you'd use this tool:

```
pseudopreset.exe ^
--output MyPreset ^
--title MyTitle ^
--author MyName ^
--level MyLevel ^
--tag myStart ^
--dream-breaker ^
--slide ^
--sun-greaves
```

Running this command will generate two files, one called `MyPreset.uasset` and
the other called `MyPreset.uexp`. Now you can package these files with your
level at the path `/Game/Mods/PseudoMenu/GamePresets` and your game preset
should appear in PseudoMenu.
