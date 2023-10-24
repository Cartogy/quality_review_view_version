extends Control

onready var cancel_button = $PanelContainer/HBoxContainer/Cancel
onready var apply_button = $PanelContainer/HBoxContainer/Apply
onready var job_line_edit = $PanelContainer/HBoxContainer/JobTypeName

onready var job_editor = $JobEditorTree

signal update_view(data)
signal new_job_row(data)

var job_type: JobHeaderData
var db: DatabaseAPI
var job_db:SQLJobDatabaseAPI

func _ready():
	db = DatabaseAPI.new()
	job_db = SQLJobDatabaseAPI.new()
	
	job_type = null
	#start_editing()
	
func set_job_type(j):
	job_type = j

func start_editing():
	if job_type == null:
		job_line_edit.text = ""
	else:
		job_line_edit.text = job_type.get_content()
		
	job_editor.start_job_editor(job_type,db.all_specifications(), job_db)
	job_editor.show_checked_sections()

func clear_tree():
	job_editor.clear_tree()


func _on_Apply_pressed():
	var job_additions_and_removals = job_editor.get_job_changes(job_db)
	var job_type = job_additions_and_removals["job_type"]
	
	if job_type == null:
		if job_line_edit.text == "":
			printerr("ERROR: Invalid job type name.")
			
			return
		else:	# Create a new job and make the additions
			var new_specs = job_additions_and_removals["adding"]
			var remove_specs = job_additions_and_removals["removing"]
			
			# Create new job
			var job_type_name = job_line_edit.text
			job_db.add_job_type(job_type_name)
			
			var job_id = job_db.get_job_type_id(job_type_name)
			
			
			if job_id != null:
				job_editor.add_new_job_specifications(job_id, new_specs, job_db)
				job_editor.remove_job_specifications(job_id, remove_specs, job_db)
				
				job_type = JobHeaderData.new()
				job_type.id = job_id
				job_type.job_name = job_type_name
				
				emit_signal("new_job_row", job_type)
			else:
				printerr("No Job ID found")
				
				return
	else:	# ADd specs and remove specs from existing job
		var new_specs = job_additions_and_removals["adding"]
		var remove_specs = job_additions_and_removals["removing"]
		
		# Update name
		if job_line_edit.text != "":
			if job_type.job_name != job_line_edit.text:
				job_type.job_name = job_line_edit.text
				
				# Update jobe type in database.
				job_db.update_job_type_name(job_type.id, job_type.job_name)
		
		job_editor.add_new_job_specifications(job_type.get_id(), new_specs, job_db)
		job_editor.remove_job_specifications(job_type.get_id(), remove_specs, job_db)
		
		
	emit_signal("update_view", job_type)
	self.hide()
	print(job_additions_and_removals)


func _on_Cancel_pressed():
	self.hide()
