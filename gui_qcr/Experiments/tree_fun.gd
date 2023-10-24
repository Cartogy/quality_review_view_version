extends Tree


func _ready():

	var root = self.create_item()
	self.set_hide_root(true)
	var child1 = self.create_item(root)
	var child2 = self.create_item(root)
	var subchild1 = self.create_item(child1)
	subchild1.set_text(0, "Subchild1")
	var subchild2 = self.create_item(child2)
	var t = TextureButton.new()
	#var t = Texture.new()
	subchild2.add_button(0,t)
	subchild2.set_button(0,0,t)
