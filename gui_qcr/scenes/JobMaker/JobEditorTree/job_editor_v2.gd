extends Control

onready var job_tree = $Tree


var root: TreeItem
var job_header: JobHeaderData

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
	
	#start_job_editor(null)
	#clear_tree()
	#acquire_checked_unchecked(root)

func start_job_editor(p_job_header: JobHeaderData, all_specifications: Array, p_db: SQLJobDatabaseAPI):
	job_header = p_job_header
	
	build_job_tree(job_tree, root, all_specifications, job_header, p_db)
	collapse_sections(root)
	

func build_job_tree(p_tree: Tree, root_item: TreeItem, all_specifications: Array, p_job_header: JobHeaderData, p_db: SQLJobDatabaseAPI):
	var all_specs = all_specifications
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
			
			# Setup specification
			var item_section = section_map[section.id]
			
			var spec_item = p_tree.create_item(item_section)
			spec_item.set_text(0, spec_data.get_content())
			
			# Set status based on job header
			if p_job_header != null:
				if p_db.job_has_specification(p_job_header.id, spec_data.id):
					spec_item.add_button(0, checked_button_texture)
					spec_item.set_checked(0,true)
					
					item_section.set_checked(0, true)
					item_section.set_button(0,0, checked_button_texture)
				else:
					spec_item.add_button(0, unchecked_button_texure)
					spec_item.set_checked(0, false)
			else:
				spec_item.add_button(0, unchecked_button_texure)
				spec_item.set_checked(0, false)
			
			# Store SpecificationData on the item.
			spec_item.set_metadata(0,spec)

		else:
			print_debug("Spec with no section")
			
			
func clear_tree():
	job_tree.clear()
	
func show_checked_sections():
	var section: TreeItem = root.get_children()
	while section != null:
		if section.is_checked(0):
			show_children(section)
		
		section = section.get_next()

func get_job_changes(p_db: SQLJobDatabaseAPI) -> Dictionary:
	return specs_to_add_remove(job_header, root, p_db)

func specs_to_add_remove(p_job_header: JobHeaderData, p_root: TreeItem, db: SQLJobDatabaseAPI) -> Dictionary:
	
	print_debug(p_job_header)
	var all_sections = []
	
	var c_section =  p_root.get_children()
	while c_section != null:
		all_sections.append(c_section)
		
		c_section = c_section.get_next()
	
	var add_remove = {}
	add_remove["adding"] = []
	add_remove["removing"] = []
	add_remove["job_type"] = p_job_header
	
	# Go over sections
	for section_item in all_sections:

		var c_spec = section_item.get_children()
		
		# Go over specifications
		while c_spec != null:
			# build the lists
			if c_spec.is_checked(0):
				var spec_data = c_spec.get_metadata(0)
				# Adding new spec to job
				if job_header == null:
					add_remove["adding"].append(spec_data)
				elif db.job_has_specification(job_header.id, spec_data.id) == false:
					add_remove["adding"].append(spec_data)
					
					

			else:
				# Removing spec from job
				var spec_data = c_spec.get_metadata(0)
				if job_header != null:
					if db.job_has_specification(job_header.id, spec_data.id):
						add_remove["removing"].append(spec_data)
					

			
			c_spec = c_spec.get_next()
	
	return add_remove
			
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

func set_check_status(p_root: TreeItem, status: bool, tex: Texture):
	var c_item: TreeItem = p_root.get_children()
	
	var b_id = 0
	var column = 0
	
	while c_item != null:
		c_item.set_checked(column, status)
		c_item.set_button(column, b_id, tex)
		
		c_item = c_item.get_next()
		
func add_new_job_specifications(job_type_id: int, new_specifications: Array, job_db: SQLJobDatabaseAPI):
	for spec in new_specifications:
		var spec_id = spec.get_id()
		
		job_db.add_job_specification(job_type_id, spec_id)
		
func remove_job_specifications(job_type_id: int, remove_specifications: Array, job_db: SQLJobDatabaseAPI):
	for spec in remove_specifications:
		var spec_id = spec.get_id()
		
		job_db.remove_job_specification(job_type_id, spec_id)

func _on_Tree_button_pressed(item: TreeItem, column, id):
	var button_id = item.get_button_by_id(column, id)
	var button = item.get_button(column,button_id)
	
	if item.is_checked(0):
		item.set_checked(0,false)
		item.set_button(0,button_id, unchecked_button_texure)
		
		set_check_status(item, false, unchecked_button_texure)
		hide_children(item)
	else:
		item.set_checked(0,true)
		item.set_button(0, button_id, checked_button_texture)
		show_children(item)
		# Ensures all checkable items are ticked.
		check_children(item)
