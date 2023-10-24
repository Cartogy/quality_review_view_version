extends Control
class_name QCReportView

# A display for godot to interact with the QCreport structure defined in Rust.

onready var questionnaire_view = $QuestionnaireView

enum report_state {EMPTY, WRITING_REPORT}
var current_state = report_state.EMPTY

var report: QCReport

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.
	$HSplitContainer/PanelContainer2/VSplitContainer/VBoxContainer.add_constant_override("separation",40)


func set_report(p_report: QCReport):

	if report != null:
		if questionnaire_view.is_connected("update_questionnaire_notes", report, "update_form_notes"):
			questionnaire_view.disconnect("update_questionnaire_notes")
		if questionnaire_view.is_connected("update_questionnaire_status", report, "update_form_status"):
			questionnaire_view.disconnect("update_questionnaire_status", report, "update_form_status")
	
	report = p_report
	
	questionnaire_view.connect("update_questionnaire_notes", report,"update_form_notes")
	questionnaire_view.connect("update_questionnaire_status",report,"update_form_status")
	questionnaire_view.exit_button.connect("pressed", self,"show_report")

func generate_pdf():
	if report != null:
		report.build_plot()
		var os_type = OS.get_name()
		match os_type:
			"Windows":
				windows_pdf()
			"X11":
				linux_pdf()
			_:
				print_debug("Not speified OS")
				
		report.write_csv(ProjectSettings.globalize_path("user://forms.csv"),
		ProjectSettings.globalize_path("user://plot.csv"))
		
func windows_pdf():
	pass
	
func linux_pdf():
	report.generate_report("testing_linux.pdf",["/usr/share/fonts/carlito/", "Carlito"])

func _on_EditReport_pressed():
	print(report.all_form_fields())
	questionnaire_view.show()
	questionnaire_view.build_questionnaire(report.all_form_fields())
	$HSplitContainer.hide()

func _on_Button_pressed():
	generate_pdf()
	
func show_report():
	$HSplitContainer.show()
	questionnaire_view.hide()
