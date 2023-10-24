extends Resource
class_name DatabaseAPI

var db: SQLDatabaseAPI

func _init():
	db = SQLDatabaseAPI.new()

## Read data from DATABASE

# [SectionData]
func all_sections() -> Array:
	var sections = []
	for raw_section in db.get_all_section_data():
		var section_data = SectionData.new()
		section_data.build(raw_section)
		
		sections.append(section_data)
	
	return sections

# [SpecificationData]
func all_specifications() -> Array:
	var specifications = []

	for raw_spec in db.get_all_specification_data():
		var spec_data = SpecificationData.new()
		spec_data.build(raw_spec)
		
		specifications.append(spec_data)
		
	return specifications

#[JobHeaderData]
func all_job_header() -> Array:
	var job_headers = []
	
	for raw_header in db.get_all_job_header_info():
		var header_data = JobHeaderData.new()
		header_data.build(raw_header)
		
		job_headers.append(header_data)
	
	return job_headers
	
	
## WRITE to DATABASE Functions	

func update_section(section: SectionData):
	print_debug("Updating section")
	db.update_section(section.get_id(),section.get_content())

func update_specification(spec: SpecificationData):
	db.update_specification(spec.id,spec.get_content(),spec.section.id)
	print_debug("Updating spec")

func update_data(data):
	print_debug("Updating data!!")

func job_has_spec(job_id: int, spec_data: SpecificationData):
	return false
