extends QcrData
class_name JobHeaderData

var id
var job_name

func build(raw_data: Dictionary):
	id = raw_data.job_id
	job_name = raw_data.job_name

func get_content():
	return job_name

func get_id():
	return id

func to_fields() -> Array:
	return [job_name]
	
func to_dictionary() -> Dictionary:
	var d = {
		"id": id,
		"content": job_name
	}
	
	return d
	
func from_dictionary(d: Dictionary):
	id = d["id"]
	job_name = d["content"]
