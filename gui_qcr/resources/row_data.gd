extends Resource
class_name RowData

var fields: Dictionary = {}

func field_count():
	return fields.size()

func add_field(key: String, field_content: String):
	fields[key] = field_content
