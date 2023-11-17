OpCapability Shader
OpExtension "SPV_KHR_terminate_invocation"
OpMemoryModel Logical GLSL450
OpEntryPoint Fragment %main "main" %frag_coord %in_data %out_data
OpExecutionMode %main OriginUpperLeft
OpDecorate %frag_coord BuiltIn FragCoord
OpDecorate %in_data Location 0
OpDecorate %in_data Flat
OpDecorate %out_data Location 0
OpDecorate %im2d DescriptorSet 0
OpDecorate %im2d Binding 0
%void = OpTypeVoid
%bool = OpTypeBool
%int = OpTypeInt 32 1
%int_0 = OpConstant %int 0
%int_1 = OpConstant %int 1
%int_8 = OpConstant %int 8
%device = OpConstant %int 1
%relaxed = OpConstant %int 0
%int2 = OpTypeVector %int 2
%int4 = OpTypeVector %int 4
%float = OpTypeFloat 32
%float4 = OpTypeVector %float 4
%ptr_int_input = OpTypePointer Input %int
%ptr_int_output = OpTypePointer Output %int
%ptr_float4_input = OpTypePointer Input %float4
%frag_coord = OpVariable %ptr_float4_input Input
%in_data = OpVariable %ptr_int_input Input
%out_data = OpVariable %ptr_int_output Output
%image = OpTypeImage %int 2D 0 0 0 2 R32i
%ptr_image_uniform = OpTypePointer UniformConstant %image
%im2d = OpVariable %ptr_image_uniform UniformConstant
%ptr_int_image = OpTypePointer Image %int
%void_fn = OpTypeFunction %void
%main = OpFunction %void None %void_fn
%entry = OpLabel
OpStore %out_data %int_1
%coord = OpLoad %float4 %frag_coord
%x_coord = OpCompositeExtract %float %coord 0
%y_coord = OpCompositeExtract %float %coord 1
%z_coord = OpCompositeExtract %float %coord 2
%x = OpConvertFToS %int %x_coord
%y = OpConvertFToS %int %y_coord
%z = OpConvertFToS %int %z_coord
%x_and_1 = OpBitwiseAnd %int %x %int_1
%y_and_1 = OpBitwiseAnd %int %y %int_1
%add = OpIAdd %int %x_and_1 %y_and_1
%ld_in_data = OpLoad %int %in_data
%combined = OpIAdd %int %add %ld_in_data
%cmp = OpIEqual %bool %combined %z
OpSelectionMerge %exit None
OpBranchConditional %cmp %then %exit
%then = OpLabel
OpTerminateInvocation
%exit = OpLabel
%im_coord = OpCompositeConstruct %int2 %x %y
%im_ptr = OpImageTexelPointer %ptr_int_image %im2d %im_coord %int_0
%old = OpAtomicIAdd %int %im_ptr %device %relaxed %x
OpReturn
OpFunctionEnd