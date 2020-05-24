use crate::Style;

pub fn match_styles(
    left: Option<Style>, right: Option<Style>
) -> Option<(Option<Style>, Option<Style>)> {
    match left {
        Some(left) => {
            match right {
                Some(right) => match_some_some(left, right),
                None => match_some_none(left)
            }
        },
        None => {
            match right {
                Some(right) => match_none_some(right),
                None => Some((None, None))
            }
        }
    }
}

fn match_some_some(
    left: Style, right: Style
) -> Option<(Option<Style>, Option<Style>)> {
    if left == right {
        if left == Style::Up || left == Style::Down {
            None
        } else {
            Some((Some(left), Some(right)))
        }
    } else if opposites(left, right) {
        Some((Some(left), Some(right)))
    } else {
        None
    }
}

fn match_some_none(
    style: Style
) -> Option<(Option<Style>, Option<Style>)> {
    match style {
        Style::Up => Some((Some(Style::Up), Some(Style::Down))),
        Style::Down => Some(
            (Some(Style::Down), Some(Style::Up))
        ),
        _ => Some((Some(style), Some(style)))
    }
}

fn match_none_some(
    style: Style
) -> Option<(Option<Style>, Option<Style>)> {
    match style {
        Style::Up => Some((Some(Style::Down), Some(Style::Up))),
        Style::Down => Some(
            (Some(Style::Up), Some(Style::Down))
        ),
        _ => Some((Some(style), Some(style)))
    }
}

fn opposites(left: Style, right: Style) -> bool {
    left == Style::Up && right == Style::Down ||
    left == Style::Down && right == Style::Up
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_none() {
        let styles = match_styles(None, None);

        assert_eq!(styles, Some((None, None)));
    }

    #[test]
    fn none_single() {
        let styles = match_styles(None, Some(Style::Single));

        assert_eq!(styles, Some((Some(Style::Single), Some(Style::Single))));
    }

    #[test]
    fn none_up() {
        let styles = match_styles(None, Some(Style::Up));

        assert_eq!(styles, Some((Some(Style::Down), Some(Style::Up))));
    }

    #[test]
    fn none_down() {
        let styles = match_styles(None, Some(Style::Down));

        assert_eq!(styles, Some((Some(Style::Up), Some(Style::Down))));
    }

    #[test]
    fn single_none() {
        let styles = match_styles(Some(Style::Single), None);

        assert_eq!(styles, Some((Some(Style::Single), Some(Style::Single))));
    }

    #[test]
    fn up_none() {
        let styles = match_styles(Some(Style::Up), None);

        assert_eq!(styles, Some((Some(Style::Up), Some(Style::Down))));
    }

    #[test]
    fn down_none() {
        let styles = match_styles(Some(Style::Down), None);

        assert_eq!(styles, Some((Some(Style::Down), Some(Style::Up))));
    }

    #[test]
    fn single_single() {
        let styles = match_styles(Some(Style::Single), Some(Style::Single));

        assert_eq!(styles, Some((Some(Style::Single), Some(Style::Single))));
    }

    #[test]
    fn single_double() {
        let styles = match_styles(Some(Style::Single), Some(Style::Double));

        assert_eq!(styles, None);
    }

    #[test]
    fn up_up() {
        let styles = match_styles(Some(Style::Up), Some(Style::Up));

        assert_eq!(styles, None);
    }

    #[test]
    fn down_down() {
        let styles = match_styles(Some(Style::Down), Some(Style::Down));

        assert_eq!(styles, None);
    }

    #[test]
    fn up_down() {
        let styles = match_styles(Some(Style::Up), Some(Style::Down));

        assert_eq!(styles, Some((Some(Style::Up), Some(Style::Down))));
    }

    #[test]
    fn down_up() {
        let styles = match_styles(Some(Style::Down), Some(Style::Up));

        assert_eq!(styles, Some((Some(Style::Down), Some(Style::Up))));
    }
}