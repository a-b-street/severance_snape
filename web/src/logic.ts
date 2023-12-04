// TODO Move to rust probably
export function classifyStep(f) {
  let props = f.properties;
  if (
    props.highway == "crossing" ||
    props.footway == "crossing" ||
    "crossing" in props
  ) {
    props.type = "crossing";
  } else if (props.highway == "footway") {
    // TODO The categories aren't mutex, some could combo
    if (props.indoor) {
      props.type = "indoors footway";
    } else if (props.layer || props.bridge || props.tunnel) {
      props.type = "footway not on the ground";
    } else {
      props.type = "footway";
    }
  } else {
    props.type = "sidewalk";
  }
}
