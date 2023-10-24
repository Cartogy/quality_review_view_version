extends Control


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var report_entry_packed = preload("res://scenes/ReportEntry/ReportEntry.tscn")
var report_view_packed = preload("res://scenes/QCReportView/QCReportView.tscn")


var report_view
var report_entry


# Called when the node enters the scene tree for the first time.
func _ready():
	var report = QCReport.new()
	report_entry = get_node("ReportEntry")
	
	report_entry.run_report_entry()
	
	report_entry.connect("start_report_review", self, "run_report_control")

func run_report_control(header_info: Dictionary, job_query: Dictionary):
	var update_header_info = {}
	
	# Change date format to be seperated into their own fields.
	for k in header_info.keys():
		if k == "date":
			var parsed_date = parse_date(header_info[k])
			update_header_info["day"] = parsed_date[0]
			update_header_info["month"] = parsed_date[1]
			update_header_info["year"] = parsed_date[2]
		else:
			update_header_info[k] = header_info[k]
	
	var new_report = QCReport.new()
	
	if report_view != null:
		report_view.queue_free()
		
	# Add the qcreport
	if get_node("QCReportView") != null:
		report_view = get_node("QCReportView")
		report_view.show()
	else:
		report_view = report_view_packed.instance()
		add_child(report_view)
	
	# Start new report review
	new_report.build_report(update_header_info, job_query)
	report_view.set_report(new_report)
	
	report_entry.queue_free()

func begin_entry():
	if report_view != null:
		report_view.queue_free()
	
	report_entry = report_entry_packed.instance()
	add_child(report_entry)
	report_entry.run_report_entry()
	
	report_entry.connect("start_report_review", self, "run_report_control")
	
func parse_date(date: String) -> Array:
	var date_format = date.split("/")
	var date_num_format = []
	for e in date_format:
		var val: String = e
		date_num_format.append(val.to_int())

	return date_num_format
