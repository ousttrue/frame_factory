#include "imgui_util.h"

namespace ImGui {

///
/// Cannot return POD struct
///
/// https://forum.dlang.org/thread/dkamxcamwttszxwwxttv@forum.dlang.org
///

void pGetContentRegionAvail(ImVec2 *pOut) {
  if (pOut) {
    *pOut = GetContentRegionAvail();
  }
}

void pGetWindowPos(ImVec2 *pOut) {
  if (pOut) {
    *pOut = GetWindowPos();
  }
}

} // namespace ImGui