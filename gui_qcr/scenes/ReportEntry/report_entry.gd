extends Control

signal start_report_review(header_info, job_query)

onready var query_start = $QueryStart
onready var error_popup = $AcceptDialog
onready var database_view = $DatabaseView
onready var button_edit_db = $EditDatabase

var db: DatabaseAPI

func _ready():
	# setup
	db = DatabaseAPI.new()
	database_view.exit_button.connect("pressed",self,"exit_database_view")
	query_start.start_button.connect("pressed", self, "start_report_review")


# Entry function
func run_report_entry():
	if db == null:
		printerr("ERROR: Not database set")
	else:
		update_job_options(db.all_job_header())


func start_report_review():
	var header_info = query_start.generate_header_info()
	var query = query_start.generate_job_query()
	
	if (valid_header_info(header_info)):
		emit_signal("start_report_review",header_info, query)
	else:
		printerr("Invalid header info")
		
	

func update_job_options(job_header: Array):
	query_start.clear_job_options()
	query_start.set_job_types(job_header)
	
	
func valid_header_info(header_info: Dictionary) -> bool:
	var errors: Array = []
	for k in header_info.keys():
		if k == "date":
			if validate_date(header_info[k]) == false:
				errors.append("invalid Date: Format must be DD/MM/YYY")
			
		else:
			var value = header_info[k]
			if value == "":
				errors.append("Invalid Entry: Please enter a value in: " + k)
	
	if errors.size() == 0:
		return true
	else:
		# Pop-up to display errors
		error_popup.set_errors(errors)
		
		error_popup.popup()
		error_popup.set_anchor(MARGIN_BOTTOM,0.5)
		error_popup.set_anchor(MARGIN_LEFT,0.5)
		error_popup.set_anchor(MARGIN_RIGHT,0.5)
		error_popup.set_anchor(MARGIN_TOP,0.5)
		return false
	
func validate_date(date: String) -> bool:
	var dd_mm_yyyy = date.split("/")
	
	if (dd_mm_yyyy.size() != 3):
		return false
		
	# check all strings are integers
	for e in dd_mm_yyyy:
		var s: String = e
		if s.is_valid_integer() == false:
			return false
			
	return true
	


func _on_EditDatabase_pressed():
	print_debug("Editing database")
	button_edit_db.hide()
	database_view.show()
	database_view.build_view()
	#query_start.hide()
	#button_edit_db.hide()

func exit_database_view():
	database_view.clear()
	database_view.hide()
	button_edit_db.show()
	query_start.show()
	
	var job_headers = db.all_job_header()
	
	update_job_options(job_headers)
