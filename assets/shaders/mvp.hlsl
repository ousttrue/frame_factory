struct VS_IN
{
    float3 Position: POSITION;
};


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
    return float4(1, 1, 1, 1);
}
