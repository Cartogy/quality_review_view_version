tool
extends Control
class_name Row

signal edit_row_sig(data,row)

export var columns = 1

const MAX_SIZE = 500

var field_scene = preload("res://scenes/Row/Field/Field.tscn")
var button_scene = preload("res://scenes/Row/ViewEditButton.tscn")

onready var hbox = $HBoxContainer
var edit_button: ViewEditButton

var attached_data: QcrData

# String
var fields: Array = []

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.
	
	
# Public

func add_field(field: String):
	fields.append(field)
	
	var field_node = field_scene.instance()
	hbox.add_child(field_node)
	field_node.set_text(field)
	field_node.rect_min_size = Vector2(_min_size_per_field(),0)


		
func build_row(p_fields: Array):

	_set_columns(p_fields.size())
	
	for f in p_fields:
		add_field(f)
		
	_add_button()
	
func clear_row():
	attached_data = null

	for n in hbox.get_children():
		hbox.remove_child(n)
	
func set_row_data(data):
	attached_data = data
####################
## PRIVATE functions
#####################


func _set_columns(cols: int):
	columns = cols

func _min_size_per_field() -> float:
	return MAX_SIZE / columns



# Adds the fields
func _build_fields(field_names: Array):
	assert(field_names.size() == columns)
	
	for key in field_names:
		add_field(key)

	_add_button()
	
func _add_button():
	var button = button_scene.instance()
	edit_button = button
	hbox.add_child(button)
	
	edit_button.connect("pressed",self,"edit_row")

func edit_row():
	print_debug("Edit row")
	emit_signal("edit_row_sig", attached_data, self)

func update_row_view(data):
	clear_row()
	set_row_data(data)
	build_row(data.to_fields())
