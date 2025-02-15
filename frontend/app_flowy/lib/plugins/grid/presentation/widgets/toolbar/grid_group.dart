import 'package:app_flowy/plugins/grid/application/field/field_controller.dart';
import 'package:app_flowy/plugins/grid/presentation/layout/sizes.dart';
import 'package:app_flowy/plugins/grid/presentation/widgets/header/field_type_extension.dart';
import 'package:flowy_infra/image.dart';
import 'package:flowy_infra/theme.dart';
import 'package:flowy_infra_ui/style_widget/button.dart';
import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flowy_infra_ui/widget/spacing.dart';
import 'package:flutter/material.dart';
import 'package:app_flowy/plugins/grid/application/setting/group_bloc.dart';

import 'package:flutter_bloc/flutter_bloc.dart';

class GridGroupList extends StatelessWidget {
  final String viewId;
  final GridFieldController fieldController;
  final VoidCallback onDismissed;
  const GridGroupList({
    required this.viewId,
    required this.fieldController,
    required this.onDismissed,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => GridGroupBloc(
        viewId: viewId,
        fieldController: fieldController,
      )..add(const GridGroupEvent.initial()),
      child: BlocBuilder<GridGroupBloc, GridGroupState>(
        builder: (context, state) {
          final cells = state.fieldContexts.map((fieldContext) {
            Widget cell = _GridGroupCell(
              fieldContext: fieldContext,
              onSelected: () => onDismissed(),
              key: ValueKey(fieldContext.id),
            );

            if (!fieldContext.canGroup) {
              cell = IgnorePointer(child: Opacity(opacity: 0.3, child: cell));
            }
            return cell;
          }).toList();

          return ListView.separated(
            shrinkWrap: true,
            itemCount: cells.length,
            itemBuilder: (BuildContext context, int index) {
              return cells[index];
            },
            separatorBuilder: (BuildContext context, int index) {
              return VSpace(GridSize.typeOptionSeparatorHeight);
            },
          );
        },
      ),
    );
  }
}

class _GridGroupCell extends StatelessWidget {
  final VoidCallback onSelected;
  final GridFieldContext fieldContext;
  const _GridGroupCell({
    required this.fieldContext,
    required this.onSelected,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = context.read<AppTheme>();

    Widget? rightIcon;
    if (fieldContext.isGroupField) {
      rightIcon = Padding(
        padding: const EdgeInsets.all(2.0),
        child: svgWidget("grid/checkmark"),
      );
    }

    return SizedBox(
      height: GridSize.typeOptionItemHeight,
      child: FlowyButton(
        text: FlowyText.medium(fieldContext.name, fontSize: 12),
        hoverColor: theme.hover,
        leftIcon: svgWidget(
          fieldContext.fieldType.iconName(),
          color: theme.iconColor,
        ),
        rightIcon: rightIcon,
        onTap: () {
          context.read<GridGroupBloc>().add(
                GridGroupEvent.setGroupByField(
                  fieldContext.id,
                  fieldContext.fieldType,
                ),
              );
          onSelected();
        },
      ),
    );
  }
}
