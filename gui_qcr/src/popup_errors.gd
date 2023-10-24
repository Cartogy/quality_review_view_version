extends AcceptDialog


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


func set_errors(errors: Array):
	var error_string = "Errors:\n"
	for e in errors:
		error_string += e + "\n"
	self.dialog_text = error_string
