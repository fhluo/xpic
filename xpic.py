import math
import os
import sys
from functools import cached_property
from pathlib import Path
from shutil import copyfile

import win32mica
from PySide6.QtCore import Qt, QMimeData, QUrl, QPoint
from PySide6.QtGui import (
    QIcon,
    QPixmap,
    QCursor,
    QAction,
    QPainter,
    QBrush,
    QColor,
    QDrag,
    QImage,
)
from PySide6.QtWidgets import (
    QVBoxLayout,
    QApplication,
    QGridLayout,
    QLabel,
    QFileDialog,
    QMenu,
    QWidget,
    QScrollArea,
)
from win32mica import MicaTheme, MicaStyle

import config
import wallpapers
from wallpapers import cache_images, get_cached_images, Size


class ContextMenu(QMenu):
    def __init__(self, parent: QWidget) -> None:
        super().__init__(parent)

        self.setWindowFlag(Qt.WindowType.FramelessWindowHint)
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)
        self.setStyleSheet(
            "background-color: rgba(7, 15, 43, 90%);" "border-radius: 4px;" "font-size: 14px;" "padding: 5px 0px;"
        )

        # open wallpaper
        self.action_open = QAction("打开", self)
        self.addAction(self.action_open)

        # save wallpaper
        self.action_save = QAction("保存", self)
        self.addAction(self.action_save)

        # set as desktop wallpaper
        self.action_set_as_desktop_wallpaper = QAction("设为桌面壁纸", self)
        self.addAction(self.action_set_as_desktop_wallpaper)

        self.popup(QCursor.pos())


def get_rounded_pixmap(path: str | os.PathLike, radius: int) -> QPixmap:
    original = QPixmap(path)

    rounded = QPixmap(original.size())
    rounded.fill(QColor(0, 0, 0, 0))

    painter = QPainter(rounded)
    painter.setRenderHint(QPainter.RenderHint.Antialiasing)
    painter.setBrush(QBrush(original))
    painter.setPen(Qt.PenStyle.NoPen)
    painter.drawRoundedRect(original.rect(), radius, radius)

    return rounded


class ImageLabel(QLabel):
    def __init__(self, path: str | os.PathLike) -> None:
        super().__init__()

        self.path = Path(path)
        self.setScaledContents(True)
        self.setPixmap(self.rounded_pixmap)
        self.setStyleSheet("background-color: transparent;")

        self.drag_start_pos: QPoint | None = None

    @cached_property
    def rounded_pixmap(self) -> QPixmap:
        return get_rounded_pixmap(self.path, 50)

    @cached_property
    def grabbed_pixmap(self) -> QPixmap:
        return self.grab()

    def open(self) -> None:
        os.startfile(self.path)

    def save(self) -> None:
        filename, _ = QFileDialog.getSaveFileName(
            parent=self,
            caption="保存",
            dir=str(Path().home() / "Pictures"),
            filter="*.jpg",
        )

        if filename != "":
            copyfile(self.path, filename)

    def set_as_desktop_wallpaper(self) -> None:
        wallpapers.set_desktop_wallpaper(self.path)

    def mouseDoubleClickEvent(self, event) -> None:
        super().mouseDoubleClickEvent(event)

        self.open()

    def contextMenuEvent(self, event) -> None:
        super().contextMenuEvent(event)

        menu = ContextMenu(self)
        menu.action_open.triggered.connect(self.open)
        menu.action_save.triggered.connect(self.save)
        menu.action_set_as_desktop_wallpaper.triggered.connect(self.set_as_desktop_wallpaper)

    def mousePressEvent(self, event) -> None:
        if event.button() == Qt.MouseButton.LeftButton:
            self.drag_start_pos = event.position().toPoint()

    def mouseMoveEvent(self, event):
        if event.buttons() != Qt.MouseButton.LeftButton:
            return

        if self.drag_start_pos is None:
            return

        if (event.position().toPoint() - self.drag_start_pos).manhattanLength() < QApplication.startDragDistance():
            return

        self.drag()

    def drag(self):
        drag = QDrag(self)

        data = QMimeData()
        data.setImageData(QImage(self.path))
        data.setUrls([QUrl(self.path.as_uri())])
        drag.setMimeData(data)

        drag.setPixmap(self.grabbed_pixmap)
        drag.exec(Qt.DropAction.CopyAction)

        self.drag_start_pos = None


class ImagesWidget(QWidget):
    def __init__(self, columns: int = 4, rows: int = 4, img_size: Size = Size(240, 135)) -> None:
        super().__init__()

        self._columns = columns
        self._rows = rows
        self.img_size = img_size

        layout = QGridLayout(self)
        layout.setSpacing(30)
        layout.setContentsMargins(50, 30, 50, 30)

        self._layout = layout

        try:
            cache_images()
        except (Exception,):
            pass

        self.images = list(get_cached_images())
        self.image_labels = [ImageLabel(img) for img in self.images]

        self.layout_images()

    @property
    def columns(self) -> int:
        return self._columns if self._columns >= 1 else 1

    @property
    def rows(self) -> int:
        return math.ceil(len(self.image_labels) / self.columns)

    @property
    def full_width(self) -> int:
        margin = self._layout.contentsMargins()
        return (
            margin.left()
            + margin.right()
            + self.columns * self.img_size.width
            + (self.columns - 1) * self._layout.spacing()
        )

    @property
    def full_height(self) -> int:
        margin = self._layout.contentsMargins()
        return (
            margin.top() + margin.bottom() + self.rows * self.img_size.height + (self.rows - 1) * self._layout.spacing()
        )

    @property
    def proper_width(self) -> int:
        return self.full_width

    @property
    def proper_height(self) -> int:
        margin = self._layout.contentsMargins()
        return (
            margin.top()
            + margin.bottom()
            + self._rows * self.img_size.height
            + (self._rows - 1) * self._layout.spacing()
        )

    @property
    def min_width(self) -> int:
        margin = self._layout.contentsMargins()
        return margin.left() + margin.right() + self.img_size.width

    @property
    def min_height(self) -> int:
        margin = self._layout.contentsMargins()
        return margin.top() + margin.bottom() + self.img_size.height

    def layout_images(self) -> None:
        for label in self.image_labels:
            self._layout.removeWidget(label)

        for i, label in enumerate(self.image_labels):
            self._layout.addWidget(label, i // self.columns, i % self.columns)

        self.setFixedSize(self.full_width, self.full_height)

    def relayout_images(self, size: Size) -> None:
        margin = self._layout.contentsMargins()
        new_columns = (size.width - margin.left() - margin.right() + self._layout.spacing()) // (
            self.img_size.width + self._layout.spacing()
        )

        if new_columns == self._columns:
            return

        self._columns = new_columns
        self.layout_images()


class MainWindow(QWidget):
    def __init__(self) -> None:
        super().__init__()

        self.setWindowTitle(config.AppName)

        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)
        scroll_area.setAlignment(Qt.AlignmentFlag.AlignCenter)
        scroll_area.setStyleSheet("background-color: transparent; border: none;")

        self.images_widget = ImagesWidget()
        scroll_area.setWidget(self.images_widget)
        scroll_area.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        scroll_area.setContentsMargins(0, 0, 0, 0)

        layout = QVBoxLayout(self)
        layout.setSpacing(0)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.addWidget(scroll_area)

        self.setMinimumWidth(self.images_widget.min_width)
        self.setMinimumHeight(self.images_widget.min_height)
        self.resize(self.images_widget.proper_width, self.images_widget.proper_height)

    def apply_mica(self) -> None:
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)

        win32mica.ApplyMica(HWND=self.winId(), Theme=MicaTheme.DARK, Style=MicaStyle.DEFAULT)

    def resizeEvent(self, event):
        self.images_widget.relayout_images(Size(event.size().width(), event.size().height()))


class App:
    def __init__(self) -> None:
        self._app = QApplication(sys.argv)
        self._app.setWindowIcon(QIcon(config.IconPath))

        with open(config.QSSPath, "r", encoding="utf-8") as f:
            self._app.setStyleSheet(f.read())

        self._window = MainWindow()

    def run(self) -> None:
        self._window.apply_mica()
        self._window.show()
        self._app.exec()


if __name__ == "__main__":
    App().run()
