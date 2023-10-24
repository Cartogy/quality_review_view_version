extends Control
class_name QueryStart

onready var vbox = $VBoxContainer
onready var hbox = $VBoxContainer/HBoxContainer

onready var header_info = $VBoxContainer/HBoxContainer/HeaderInfo
onready var additions = $VBoxContainer/HBoxContainer/Additions
onready var job_options = $VBoxContainer/HBoxContainer/VBoxContainer/AspectRatioContainer/JobType
onready var start_button = $VBoxContainer/StartReview

onready var engineer_label = $VBoxContainer/HBoxContainer/HeaderInfo/EngineerContainer/Engineer
onready var date_label = $VBoxContainer/HBoxContainer/HeaderInfo/DateContainer/Date
onready var job_label = $VBoxContainer/HBoxContainer/HeaderInfo/JobContainer/Job
onready var location_label = $VBoxContainer/HBoxContainer/HeaderInfo/LocationContainer/Location

# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	vbox.add_constant_override("separation", 30)
	hbox.add_constant_override("separation", 40)
	#test_options()


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

#[JobHeaderData]
func set_job_types(jobs: Array):
	for j in jobs:
		var job_data: JobHeaderData = j
		
		job_options.add_item(job_data.get_content(), job_data.get_id())

func clear_job_options():
	job_options.clear()

func test_options():
	job_options.add_item("Cement", 0)
	job_options.add_item("Horizontal",1)

func generate_header_info():
	var d = {}
	for e in header_info.get_children():
		var line:LineEdit = e.get_child(0)
		d[line.key_name] = line.text
	
	return d
	
func generate_job_query() -> Dictionary:
	var d = {}
	d["job_name"] = job_options.get_item_text(job_options.selected)
	d["job_id"] = job_options.get_item_id(job_options.selected)
	
	var adds = []
	for e in additions.get_children():
		var check: CheckBox = e
		if check.pressed:
			adds.append(check.text)
			
	d["additional_sections"] = adds
	return d

# Uses to reset all the values in the query.
func reset():
	job_options.select(0)
	# Change additions
	for e in additions.get_children():
		e.pressed = false
	
	# reset header info
	for e in header_info.get_children():
		
		var line:LineEdit = e.get_child(0)
		line.text = ""
