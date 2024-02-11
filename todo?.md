MVP stuff:
- basic gui in rust that allows you to generate a map
- user can choose minimum distance for shortest path through dungeon
- user can export generated map to file in some way

## ideas to expand on at some point
- generate dungeon with multiple levels, plus multiple ways to traverse levels
- on generating dungeons with multiple levels, could have controls to determine what type of generation to use on different levels


## how to represent maps, in program + in files/export
- so, I think the way things are represented should potentially different depending on whether we're generating caves or rooms
- if caves, then we should use a similar method to cave layout with schema paper
	- use initial sketch on something like several 10x10 tiles
	- for gui of this, could define maximum height change in one tile, plus steps
	- then, have a number of starting tiles. from these, you can click on tiles adjacent to choose height change for each one
	- for initial map, use grayscale colors to indicate height to user, just like in the paper
	- after doing this for all levels, program will place them on top of each other, and then show where levels connect with color
	- there might be tiles that are already close enough to connect, and the user can toggle connections whichever way they want on each tile
	- from there, use cellular automata to generate cave-live curves and look of image, as in paper
	- the eventual shape and payout of caves would likely need to be pixel-coordinate-based, though we can have the user define how big they want the caves to be. looping structures is important here
- if rooms, then we should start by defining the shape and size of the bounds for the structure
	- we'll also want to determine the number of floors in the structure, as well as potentially which floors are considered ground floors
	- we'll want to categorize the entrances/exits by where they lead. we'll have exits going up, exits going down, and exits going outside
	- for each floor, we'll want to fill the space with rooms, defined by their walls
	- the rooms might be represented on a grid to the user, with different rooms being represented by different structures
	- we'll want a universal unit that the user can define for how big the whole structure is, relative to the size of the cave structures
	- from there, we'll also have a control for how granular the grid for showing rooms is. That allows the user to make blocky rooms rather easily with a smaller grid, or, if they want more granular control of the shape, they can increase the resolution
	- once we have rooms, we'll want to define whether rooms connect
		- to do this, we migth first generate a list of potential room connections
		- from here, we might go through each room and randomly determine whether a connection exists
		- from there, we could generate a list of possible shortest paths from a entrance of one type to an entrance of a different type. These might be categorized as up-paths, down-paths, and exit-paths. 
		- from these path-lists, randomly make a few of the shortest paths secret
		- to ensure a path from the top to the bottom can be found without secrets, attempt to find a path from top to bottom, and if one cannot be found, then try removing secrets until such a path can be found
		- we should maybe also have functions that bias towards looping corridors, but that can come later
- if both rooms and caves, then generate rooms and caves separately, then allow user to choose where room-based structures intersect with caves
	- after this, we'll have to determine where caves connect with rooms, and then turn those connections into either normal or secret doors

## Brainstorm tabs to include
- tab for defining input map for cave
	- include controls for number of levels, maybe grid shows one level at a time, and you can flip through them
- tab for defining room map for rooms
	- similar level control as for caves
- tab for defining settings for room and cave generation plus misc settings
- tab for showing output image, drawn on canvas, but held within scrollable element so you can zoom in maybe
- tab for defining where cave map should be in relation to rooms, or maybe just lets you place room and cave maps in a larger space and determines connections for all

let's expand/break down stuff

# basic gui in rust that allows you to generate a map
- several aspects of gui should be designed to make things easier, with different sections
	- tab for settings for generation
	- tab for viewing list of loaded maps
		- need to add option to import and save maps to file
	- tab for viewing a map
		- need to figure out either a button grid or export and display a picture or something
		- if button grid, could use buttons for editing, might need to be careful about performance through
		- if button grid, might also need to have ability to zoom in or pan view of map, instead of just seeing whole map at once.

# user can choose minimum distance for shortest path through dungeon
- the user should probably be able to set the shortest path via the settings page
- there are a few ways of ensuring a minimum distance, depending on how generation works
	- keep track of minimum distance during generation, and keep generating more content until a minimum distance is achieved.
	- generate the whole map, and then at the end, block random corridors/create obstacles until the minimum distance is achieved
	- maybe we could let the user choose multiple methods for ensuring minimum distance?

# how to actually generate maps?
- should probably have multiple generation techniques for different structures: caves, buildings, ruins, whatev

# brainstorm settings to include
- how many rooms can be non-euclidian, maybe as percentage
- how much empty space can be present on the map image, maybe as percentage
	- by empty space, I refer to the coordinates on the space which is not taken up by a room, corridor, wall, door, etc
- have list of allowed obstacles to use in map
- maybe we could let the user choose multiple methods for ensuring minimum distance? Or multiple generation techniques

