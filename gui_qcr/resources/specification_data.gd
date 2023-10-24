extends QcrData
class_name SpecificationData

var id
var content

var section: SectionData

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func build(raw_data: Dictionary):
	content = raw_data.specification_content
	id = raw_data.specification_id
	section = SectionData.new()
	section.id = raw_data.section.section_id
	section.section_name = raw_data.section.section_name

func get_content():
	return content
	
func get_id():
	return id

# -> [String]
func to_fields() -> Array:
	return [section.section_name,content]
	
func to_dictionary() -> Dictionary:
	return {
		"id":id,
		"content": content,
		"section_id": section.id,
		"section_name": section.section_name
	}

func from_dictionary(d: Dictionary):
	id = d["id"]
	content = d["content"]
	var l_section = SectionData.new()
	l_section.id = d["section_id"]
	l_section.section_name = d["section_name"]
	
	section = l_section
	
	
