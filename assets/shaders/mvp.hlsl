struct VS_IN {
  float3 Position : POSITION;
  float3 Normal : NORMAL;
  float2 Tex : TEXCOORD;
};

Texture2D diffuseTexture;
SamplerState diffuseTextureSampler;

cbuffer frame {
  // frame
  row_major matrix ViewMatrix;
  row_major matrix ProjectionMatrix;
  float4 LightDirection;
  // node
  row_major matrix ModelMatrix;
  // submesh
  float4 Color;
}

struct PS_IN {
  float4 Position : SV_POSITION;
  float2 Tex : TEXCOORD;
  float4 Color : COLOR;
};

PS_IN vsMain(VS_IN input) {
  PS_IN output;
  output.Position = mul(float4(input.Position, 1),
                        mul(ModelMatrix, mul(ViewMatrix, ProjectionMatrix)));
  output.Tex = input.Tex;

  // lighting
  float shading = max(1, dot(input.Normal, -LightDirection) + 0.5);
  output.Color.rgb = Color.rgb * shading;
  output.Color.a = Color.a;

  return output;
}

float4 psMain(PS_IN input) : SV_TARGET {
  float2 tex = float2(input.Tex.x, 1 - input.Tex.y);
  float4 texel = diffuseTexture.Sample(diffuseTextureSampler, tex);
  return texel * input.Color;
}
