extends Control
class_name DatabaseView

# The handler must convert the raw data into the RowData array.

onready var job_view = $TabSelection/TabContainer/Jobs
onready var section_view = $TabSelection/TabContainer/Sections
onready var specification_view = $TabSelection/TabContainer/Specifications

onready var specification_editor = $SpecificationEditor
onready var section_editor = $SectionEditor
onready var job_maker = $JobMaker

onready var exit_button = $TabSelection/Exit

var db: DatabaseAPI

func _ready():
	#test_view()
	db = DatabaseAPI.new()
	
	section_editor.connect("update_database",db,"update_section")
	specification_editor.connect("update_database",db,"update_specification")
	specification_editor.db = db

func build_view():

	var sections: Array = db.all_sections()
	var jobs: Array = db.all_job_header()
	var specs: Array = db.all_specifications()
	

	section_view.build_rows(sections, section_editor)

	specification_view.build_rows(specs, specification_editor)
	
	job_view.build_rows(jobs, null)
	job_maker.connect("new_job_row", self, "create_job_row")

	for job_rows in job_view.get_rows():
		job_rows.connect("edit_row_sig",self,"edit_job")
	
	#build_job_view(jobs)
	#build_section_view(sections)
	#build_specification_view(specs)


func clear():
	job_view.clear()
	section_view.clear()
	specification_view.clear()



##################
# Testing unit
##################

func test_view():
	var rd0 = RowData.new()
	rd0.fields = ["Cement"]
	var rd1 = RowData.new()
	rd1.fields = ["Horizontal"]
	var rd11 = RowData.new()
	rd11.fields = ["Horizontal"]
	var rd12 = RowData.new()
	rd12.fields = ["Horizontal"]
	var rd13 = RowData.new()
	rd13.fields = ["Horizontal"]
	var rd14 = RowData.new()
	rd14.fields = ["Horizontal"]
	var rd15 = RowData.new()
	rd15.fields = ["Horizontal"]
	
	var job_data = [rd0,rd1,rd11,rd12,rd13,rd14,rd15]
	
	var rd2 = RowData.new()
	rd2.fields = ["Cover Page"]
	var rd3 = RowData.new()
	rd3.fields = ["Well Data"]
	
	var section_data = [rd2,rd3]
	
	var rd4 = RowData.new()
	rd4.fields = ["Cover Page", "Title"]
	var rd5 = RowData.new()
	rd5.fields = ["Well Data", "Well Percentage"]

	var specification_data = [rd4,rd5]

func edit_section(data):
	section_editor.prompt_editor(data)

func edit_specification(data):
	specification_editor.prompt_editor(data)
	
func edit_job(p_data, row:Row):
	# Clean set of signals. No need to disconnect them manually.
	if job_maker.is_connected("update_view", row, "update_row_view"):
		job_maker.disconnect("update_view", row, "update_row_view")
	row.clear_row()
	
	job_maker.connect("update_view", row, "update_row_view")
	job_maker.show()
	
	print_debug(p_data.get_id())
	
	job_maker.set_job_type(p_data)
	job_maker.start_editing()

func create_job_row(p_data: QcrData):
	var new_row = job_view.add_row(p_data, null)
	new_row.connect("edit_row_sig",self,"edit_job")
	

func _on_New_Specification_pressed():
	specification_editor.prompt_editor(null)


func _on_New_Section_pressed():
	section_editor.prompt_editor(null)


func _on_New_Job_pressed():
	job_maker.set_job_type(null)
	job_maker.show()
	job_maker.start_editing()
