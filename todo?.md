MVP stuff:
- basic gui in rust that allows you to generate a map
- user can choose minimum distance for shortest path through dungeon
- user can export generated map to file in some way

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

