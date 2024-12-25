#version 100
precision highp float;
#define UNIFORMARRAY_SIZE 11

varying vec2 pos;
varying vec2 coord;

uniform sampler2D aTexture;
uniform vec4 aFrag[UNIFORMARRAY_SIZE];

#define type int(aFrag[0].x)
#define texType int(aFrag[0].y)
#define radius aFrag[0].z
#define feather aFrag[0].w

#define strokeMult aFrag[1].x
#define strokeThr aFrag[1].y
#define extent aFrag[1].zw

#define scissorMat mat3(aFrag[2].xyz,aFrag[3].xyz,aFrag[4].xyz)
#define scissorExt aFrag[5].xy
#define scissorScale aFrag[5].zw

#define paintMat mat3(aFrag[6].xyz,aFrag[7].xyz,aFrag[8].xyz)

#define innerCol aFrag[9]
#define outerCol aFrag[10]

float scissorMask(vec2 pos){
  vec2 sc=(abs((scissorMat*vec3(pos,1.)).xy)-scissorExt);
  sc=vec2(.5,.5)-sc*scissorScale;
  return clamp(sc.x,0.,1.)*clamp(sc.y,0.,1.);
}

float strokeMask(void){
  return min(1.,(1.-abs(coord.x*2.-1.))*strokeMult)*min(1.,coord.y);
}

void main(void)
{
  float strokeAlpha=strokeMask();
  float scissor=scissorMask(pos);
  
  if(type==0){
    //FillGradient
    vec4 c=vec4(innerCol.xyz,innerCol.w*strokeAlpha);
    gl_FragColor=c;
  }
  else if(type==1){
    //FillTexture
    vec2 pt=(paintMat*vec3(pos,1.)).xy/extent;
    vec4 color=texture2D(aTexture,pt);
    // Apply color tint and alpha.
    color*=innerCol;
    // Combine alpha
    color*=strokeAlpha*scissor;
    gl_FragColor=color;
  }
  else if(type==2){
    // FillStencil
    gl_FragColor=vec4(1,1,1,1);
  }
  else if(type==3){
    // FillAlpha
    vec2 pt=(paintMat*vec3(pos,1.)).xy/extent;
    vec4 color=texture2D(aTexture,pt);
    gl_FragColor=vec4(innerCol.xyz,color.w);
  }
}
