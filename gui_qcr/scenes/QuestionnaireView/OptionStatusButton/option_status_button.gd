extends OptionButton
class_name OptionStatusButton

signal update_form_status(form_id, status)

var unit_form_id

func _on_OptionStatusButton_item_selected(index):
	emit_signal("update_form_status", unit_form_id, index)
