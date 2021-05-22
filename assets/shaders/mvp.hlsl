struct VS_IN {
  float3 Position : POSITION;
  float3 Normal : NORMAL;
  float2 Tex : TEXCOORD;
};

Texture2D diffuseTexture;
SamplerState diffuseTextureSampler;

cbuffer frame {
  row_major matrix ViewMatrix;
  row_major matrix ProjectionMatrix;
}

cbuffer model { row_major matrix ModelMatrix; }

struct PS_IN {
  float4 Position : SV_POSITION;
  float3 Normal: NORMAL;
  float2 Tex : TEXCOORD;
};

PS_IN vsMain(VS_IN input) {
  PS_IN output;
  output.Position = mul(float4(input.Position, 1),
                        mul(ModelMatrix, mul(ViewMatrix, ProjectionMatrix)));
output.Normal = input.Normal;
  output.Tex = input.Tex;
  return output;
}

float4 psMain(PS_IN input) : SV_TARGET {
    float2 tex=float2(input.Tex.x, 1-input.Tex.y);
  float4 texel = diffuseTexture.Sample(diffuseTextureSampler, tex);
  return texel;
}
