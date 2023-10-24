extends Control

onready var job_tree = $Tree


var root: TreeItem
var db: DatabaseAPI

var checked_button_texture: Texture
var unchecked_button_texure: Texture

func _ready():
	
	var tmp_button = CheckBox.new()
	
	# SEt up buttons 
	#print(OS.get_window_size())
	#var x_win = OS.get_window_size().x
	#hbox.add_constant_override("separation",x_win / 3)
	
	checked_button_texture = tmp_button.get_icon("checked")
	unchecked_button_texure = tmp_button.get_icon("unchecked")
	
	root = job_tree.create_item()
	
	build_job_tree(job_tree, root, DatabaseAPI.new())
	collapse_sections(root)
	
	acquire_checked_unchecked(root)

func build_job_tree(p_tree: Tree, root_item: TreeItem, p_db: DatabaseAPI):
	var all_specs = p_db.all_specifications()
	var section_map: Dictionary = {}
	
	# Add the specs in their sections
	for spec in all_specs:
		var spec_data: SpecificationData = spec
		var section: SectionData = spec_data.section
		
		# Only add specs with a section
		if section != null:
			
			# Create a section Item Tree
			if section_map.has(section.id) == false:
				# Prepare the section Item Tree
				var item_section: TreeItem = p_tree.create_item(root_item)
				item_section.set_text(0, section.get_content())
				item_section.add_button(0, unchecked_button_texure)
				section_map[section.id] = item_section
			
			# Create specification item
			var item_section = section_map[section.id]
			var spec_item = p_tree.create_item(item_section)
			spec_item.set_text(0, spec_data.get_content())
			spec_item.add_button(0, unchecked_button_texure)
			
			# Store SpecificationData on the item.
			spec_item.set_metadata(0,spec)
		else:
			print_debug("Spec with no section")
			
func acquire_checked_unchecked(p_root: TreeItem):
	
	var all_sections = []
	
	var c_section =  p_root.get_next()
	while c_section != null:
		all_sections.append(c_section)
		
		c_section = c_section.get_next()
	
	var checked_list = []
	var unchecked_list = []
	
	for section_item in all_sections:
		var c_spec = section_item.get_next()
		
		while c_spec != null:
			# build the lists
			if c_spec.is_checked(0):
				checked_list.append(c_spec.get_metadata(0))
			else:
				unchecked_list.append(c_spec.get_metadata(0))
			
			c_spec = c_spec.get_next()
		
	print_debug(checked_list)
	print_debug(unchecked_list)
			
func collapse_sections(p_root: TreeItem):
	var c_section: TreeItem = p_root.get_children()
	# Iterate through all section items
	while c_section != null:
		hide_children(c_section)
		c_section = c_section.get_next()

func hide_children(item: TreeItem):
	item.collapsed = true
	item.disable_folding = true

func show_children(item: TreeItem):
	item.collapsed = false
	item.disable_folding = false

# Used to show sections
func check_children(p_root: TreeItem):
	var c_item: TreeItem = p_root.get_children()
	
	var b_id = 0
	var column = 0
	# Check all children to be marked
	while c_item != null:
		c_item.set_checked(column, true)
		c_item.set_button(column,b_id,checked_button_texture)
		
		c_item = c_item.get_next()

func _on_Tree_button_pressed(item: TreeItem, column, id):
	var button_id = item.get_button_by_id(column, id)
	var button = item.get_button(column,button_id)
	
	if item.is_checked(0):
		item.set_checked(0,false)
		item.set_button(0,button_id, unchecked_button_texure)
		hide_children(item)
	else:
		item.set_checked(0,true)
		item.set_button(0, button_id, checked_button_texture)
		show_children(item)
		# Ensures all checkable items are ticked.
		check_children(item)
