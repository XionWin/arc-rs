#version 100

attribute vec2 aPos;

uniform vec2 aViewport;
uniform int aPointSize;

void main(void)
{
    gl_Position = vec4(aPos.x / aViewport.x, aPos.y / aViewport.y, 0.0, 1.0);
	gl_PointSize = float(aPointSize);
}