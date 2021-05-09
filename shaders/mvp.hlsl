struct VS_IN
{
    float3 Position: POSITION;
};

// each frame
cbuffer cb0
{
	row_major matrix ViewMatrix;
	row_major matrix ProjectionMatrix;
}

// each model
cbuffer cb1
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
	output.Position = float4(input.Position, 1);
    return output;    
}

float4 psMain(PS_IN input) : SV_TARGET
{
    return float4(1, 1, 1, 1);
}
