#version 450

layout(location = 0) in vec3 v_WorldPosition;
layout(location = 1) in vec3 v_WorldNormal;
layout(location = 2) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;


layout(set = 2, binding = 0) uniform IrrelevantMaterial_irrelevant{
    float irrelevant;
};



///////////////////////////////////////////////////////////////////////////////////////////////7
float cloudscale = 0.6;//1.1;
float speed = 0.03;
float clouddark =6.0;// 0.5;
float cloudlight = 0.3;
float cloudcover = 0.0;
float cloudalpha = 1.0;
float skytint = 19.0;
vec3 skycolour1 = vec3(0.2, 0.4, 0.6);
vec3 skycolour2 = vec3(0.12, 0.20, 0.43);

const mat3 m = mat3( 1.0,  0.0, 0.0,  0.0 , 1.0,0.0,0.0,0.0,1.0);

vec2 hash( vec2 p ) {
	p = vec2(dot(p,vec2(127.1,311.7)), dot(p,vec2(269.5,183.3)));
	return -1.0 + 2.0*fract(sin(p)*43758.5453123);
}

float mod289(float x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 mod289(vec4 x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 perm(vec4 x){return mod289(((x * 34.0) + 1.0) * x);}

float noise(vec3 p){
    vec3 a = floor(p);
    vec3 d = p - a;
    d = d * d * (3.0 - 2.0 * d);

    vec4 b = a.xxyy + vec4(0.0, 1.0, 0.0, 1.0);
    vec4 k1 = perm(b.xyxy);
    vec4 k2 = perm(k1.xyxy + b.zzww);

    vec4 c = k2 + a.zzzz;
    vec4 k3 = perm(c);
    vec4 k4 = perm(c + 1.0);

    vec4 o1 = fract(k3 * (1.0 / 41.0));
    vec4 o2 = fract(k4 * (1.0 / 41.0));

    vec4 o3 = o2 * d.z + o1 * (1.0 - d.z);
    vec2 o4 = o3.yw * d.x + o3.xz * (1.0 - d.x);

    return (o4.y * d.y + o4.x * (1.0 - d.y))*0.2;
}

float fbm(vec3 n) {
	float total = 0.0, amplitude = 0.1;
	for (int i = 0; i < 7; i++) {
		total += noise(n) * amplitude;
		n = m * n;
		amplitude *= 0.4;
	}
	return total;
}


void main() {
	vec3 uv = v_WorldPosition*0.1;    
    float q = fbm(uv * cloudscale * 0.5);
    
    //ridged noise shape
	float r = 0.0;
	uv *= cloudscale;
    uv -= q ;
    float weight = 3.4999;//0.8;
    for (int i=0; i<8; i++){
		r += abs(weight*noise( uv ));
        uv = m*uv;
		weight *= 0.7;
    }
    
    //noise shape
	float f = 0.0;
    uv = v_WorldPosition*0.05;
	uv *= cloudscale;
    uv -= q ;
    weight = 0.7;
    for (int i=0; i<8; i++){
		f += weight*noise( uv );
        uv = m*uv ;
		weight *= 0.6;
    }
    
    f *= r + f;
    
    
    vec3 skycolour = mix(skycolour2, skycolour1,v_Uv.y);
    vec3 cloudcolour = vec3(0.9, 0.9, 0.7) * clamp((clouddark + cloudlight), 0.0, 1.0);
   
    f = cloudcover + cloudalpha*f*r;
    
    vec3 result = mix(skycolour, clamp(skytint * skycolour + cloudcolour, 0.0, 1.0), clamp(f , 0.0, 1.0));
    
	o_Target  = vec4(result, 1.0 );
}