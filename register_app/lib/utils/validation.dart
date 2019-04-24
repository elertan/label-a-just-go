final isValidUuidv4 = (String str) {
  final exp = RegExp(r"[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}");
  return exp.hasMatch(str);
};