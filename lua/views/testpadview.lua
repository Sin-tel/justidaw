local audiolib = require("audiolib")

testPadView = View:derive("TestPad")

function testPadView:new()
	local new = {}
	setmetatable(new, self)
	self.__index = self

	new.v = 0
	new.f = 49

	new.note = false

	return new
end

function testPadView:draw()
	local w, h = self:getDimensions()
	local mx, my = self:getMouse()

	local x1 = w * 0.05
	local y1 = h * 0.05
	local x2 = w * 0.95
	local y2 = h * 0.95

	love.graphics.setColor(theme.bg_nested)
	love.graphics.rectangle("fill", x1, y1, x2 - x1, y2 - y1)

	love.graphics.setColor(theme.widget_line)
	love.graphics.rectangle("line", x1, y1, x2 - x1, y2 - y1)

	local oct = math.floor(w / 200)
	if oct < 1 then
		oct = 1
	end
	for i = 1, oct - 1 do
		xx = x1 + i * (x2 - x1) / oct
		love.graphics.line(xx, y1, xx, y2)
	end

	love.graphics.setColor(theme.ui_text)

	mx = clamp(mx, x1, x2)
	my = clamp(my, y1, y2)

	if self.box.focus then
		love.graphics.ellipse("line", mx, my, 5)

		local mxx = (mx - x1) / (x2 - x1)
		local myy = (my - y1) / (y2 - y1)

		self.f = 60 - math.floor(oct * 0.5) * 12 + oct * 12 * mxx

		self.v = 1.0 - myy

		if mouse.button == 1 or mouse.button == 2 then
			audiolib.send_CV(selection.channel.index, self.f, self.v)
		end
	end
end

function testPadView:mousepressed()
	if mouse.button == 1 or mouse.button == 2 and selection.channel then
		audiolib.send_noteOn(selection.channel.index, self.f, self.v)
		self.note = true
	end
end

function testPadView:mousereleased()
	if mouse.button == 1 and selection.channel then
		audiolib.send_CV(selection.channel.index, self.f, 0)
		self.note = false
	end
end
