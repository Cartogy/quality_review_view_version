extends Control

onready var tree = $Tree

var root

func _ready():
	root = tree.create_item()

func build_questionnaire(unit_forms: Array):
	for form in unit_forms:
		var section_name = form[1]
		var specification_content = form[2]
		
		var row_item: TreeItem = tree.create_item(root)
		
		row_item.set_text(0, section_name)
		row_item.set_editable(0, true)
		row_item.set_text(1, specification_content)
		row_item.set_editable(1, true)
		row_item.set_expand_right(1, true)
		row_item.set_text(2, "")
		row_item.set_editable(2, true)
		row_item.set_expand_right(2, true)
		row_item.set_cell_mode(3, TreeItem.CELL_MODE_RANGE)
