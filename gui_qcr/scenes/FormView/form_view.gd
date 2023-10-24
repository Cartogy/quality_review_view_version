extends MarginContainer
class_name FormView

onready var vbox = $Panel/ScrollContainer/VBoxContainer

onready var row_packed = preload("res://scenes/Row/Row.tscn")

func _ready():
	vbox.add_constant_override("separation",40)

#[QcrData]
func build_rows(qcrdatas: Array, editor: FieldEditor):
	for data in qcrdatas:
		# specify the type
		var data_qcr: QcrData = data

		# Prepare row scene
		var row = row_packed.instance()
		vbox.add_child(row)
		
		row.set_row_data(data_qcr)

		row.build_row(data_qcr.to_fields())
		
		if editor != null:
			row.connect("edit_row_sig",editor,"prompt_editor")

func get_rows():
	return vbox.get_children()
	
func add_row(data: QcrData, editor: FieldEditor) -> Row:
	var row = row_packed.instance()
	vbox.add_child(row)
	row.set_row_data(data)
	
	row.build_row(data.to_fields())
	
	if editor != null:
		row.connect("edit_row_sig", editor, "prompt_editor")
		
	return row

func clear():
	var rows = get_rows()
	for row in rows:
		vbox.remove_child(row)

