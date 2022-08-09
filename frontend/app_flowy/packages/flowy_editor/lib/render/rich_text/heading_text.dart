import 'package:flowy_editor/document/node.dart';
import 'package:flowy_editor/editor_state.dart';
import 'package:flowy_editor/render/rich_text/default_selectable.dart';
import 'package:flowy_editor/render/rich_text/flowy_rich_text.dart';
import 'package:flowy_editor/render/rich_text/rich_text_style.dart';
import 'package:flowy_editor/render/selection/selectable.dart';
import 'package:flowy_editor/service/render_plugin_service.dart';
import 'package:flutter/material.dart';

class HeadingTextNodeWidgetBuilder extends NodeWidgetBuilder<TextNode> {
  @override
  Widget build(NodeWidgetContext<TextNode> context) {
    return HeadingTextNodeWidget(
      key: context.node.key,
      textNode: context.node,
      editorState: context.editorState,
    );
  }

  @override
  NodeValidator<Node> get nodeValidator => ((node) {
        return node.attributes.heading != null;
      });
}

class HeadingTextNodeWidget extends StatefulWidget {
  const HeadingTextNodeWidget({
    Key? key,
    required this.textNode,
    required this.editorState,
  }) : super(key: key);

  final TextNode textNode;
  final EditorState editorState;

  @override
  State<HeadingTextNodeWidget> createState() => _HeadingTextNodeWidgetState();
}

// customize

class _HeadingTextNodeWidgetState extends State<HeadingTextNodeWidget>
    with Selectable, DefaultSelectable {
  final _richTextKey = GlobalKey(debugLabel: 'heading_text');
  final topPadding = 5.0;
  final bottomPadding = 2.0;

  @override
  Selectable<StatefulWidget> get forward =>
      _richTextKey.currentState as Selectable;

  @override
  Offset get baseOffset {
    return Offset(0, topPadding);
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: maxTextNodeWidth,
      child: Padding(
        padding: EdgeInsets.only(
          top: topPadding,
          bottom: bottomPadding,
        ),
        child: FlowyRichText(
          key: _richTextKey,
          placeholderText: 'Heading',
          placeholderTextSpanDecorator: _placeholderTextSpanDecorator,
          textSpanDecorator: _textSpanDecorator,
          textNode: widget.textNode,
          editorState: widget.editorState,
        ),
      ),
    );
  }

  TextSpan _textSpanDecorator(TextSpan textSpan) {
    return TextSpan(
      children: textSpan.children
          ?.whereType<TextSpan>()
          .map(
            (span) => TextSpan(
              text: span.text,
              style: span.style?.copyWith(
                fontSize: widget.textNode.attributes.fontSize,
              ),
              recognizer: span.recognizer,
            ),
          )
          .toList(),
    );
  }

  TextSpan _placeholderTextSpanDecorator(TextSpan textSpan) {
    return TextSpan(
      children: textSpan.children
          ?.whereType<TextSpan>()
          .map(
            (span) => TextSpan(
              text: span.text,
              style: span.style?.copyWith(
                fontSize: widget.textNode.attributes.fontSize,
              ),
              recognizer: span.recognizer,
            ),
          )
          .toList(),
    );
  }
}
