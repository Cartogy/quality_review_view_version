extends QcrData
class_name SectionData

var id
var section_name

func build(raw_data: Dictionary):
	id = raw_data.section_id
	section_name = raw_data.section_name

func get_content():
	return section_name
	
func get_id():
	return id

func to_fields() -> Array:
	return [section_name]
	
func to_dictionary() -> Dictionary:
	var d = {
		"id": id,
		"content": section_name
	}
	
	return d
	
func from_dictionary(d: Dictionary):
	id = d["id"]
	section_name = d["content"]
