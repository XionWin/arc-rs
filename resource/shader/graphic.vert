#version 100

attribute vec2 aPos;
attribute vec2 aCoord;

uniform vec2 aOffset;
uniform vec2 aViewport;

varying vec2 pos;
varying vec2 coord;

void main(void)
{
	gl_Position=vec4((aPos.x+aOffset.x)/aViewport.x*2.-1.,1.-(aPos.y+aOffset.y)/aViewport.y*2.,0.,1.);
	coord=aCoord;
	pos=aPos;
}