struct VS_IN
{
    float3 Position: POSITION;
};

Texture2D diffuseTexture;
SamplerState diffuseTextureSampler;


cbuffer frame
{
	row_major matrix ViewMatrix;
	row_major matrix ProjectionMatrix;
}


cbuffer model
{
	row_major matrix ModelMatrix;
}

struct PS_IN
{
    float4 Position: SV_POSITION;
};

PS_IN vsMain(VS_IN input)
{
    PS_IN output;
	output.Position = mul(float4(input.Position, 1), mul(ModelMatrix, mul(ViewMatrix, ProjectionMatrix)));
    return output;    
}

float4 psMain(PS_IN input) : SV_TARGET
{
	float4 texel = diffuseTexture.Sample(diffuseTextureSampler, float2(0.5, 0.5));
    return texel;
}
