extern crate nalgebra as na;
extern crate image;
extern crate windows;
extern crate rand;

use windows::Win32::{Graphics::Gdi::*, Foundation::*, UI::WindowsAndMessaging::*, UI::Input::KeyboardAndMouse::*, System::LibraryLoader::*};
use windows::s;

struct Camera
{
	position: na::Point3<f32>,
	rotation: na::UnitQuaternion<f32>,
	perspective: na::Perspective3<f32>
}

struct Triangle
{
	p1: na::Point3<f32>,
	p2: na::Point3<f32>,
	p3: na::Point3<f32>
}
impl Triangle
{
	fn new(p1: na::Point3<f32>, p2: na::Point3<f32>, p3: na::Point3<f32>) -> Self
	{
		return Triangle
		{
			p1: p1.clone(),
			p2: p2.clone(),
			p3: p3.clone()
		};
	}

	fn render(&self, camera: &Camera, pixels: &mut Vec<u8>, depth_buffer: &mut Vec<f32>)
	{
		let p1 = Triangle::get_point(&self.p1, &camera);
		let p2 = Triangle::get_point(&self.p2, &camera);
		let p3 = Triangle::get_point(&self.p3, &camera);

		Triangle::draw_triangle(&p1, &p2, &p3, pixels, depth_buffer);
		Triangle::draw_line(&p1, &p2, pixels);
		Triangle::draw_line(&p2, &p3, pixels);
		Triangle::draw_line(&p3, &p1, pixels);
	}

	fn get_point(p: &na::Point3<f32>, camera: &Camera) -> na::Point3<f32>
	{
		let forward = na::vector![0.0f32, 0.0, -1.0];
		let up = na::vector![0.0f32, 1.0, 0.0];
		let look_dir = camera.rotation * forward;
		let up_dir = camera.rotation * up;
		let target_pos = camera.position + look_dir;
		let view = na::Matrix4::look_at_rh(&camera.position, &target_pos, &up_dir);

		let p_transformed = view.transform_point(&p);
		let p_projected = camera.perspective.project_point(&p_transformed);
		return p_projected;
	}

	fn draw_line(p1: &na::Point3<f32>, p2: &na::Point3<f32>, pixels: &mut Vec<u8>)
	{
		let x1 = ((p1.x + 1.0) * (1024.0 / 2.0)).floor() as i32;
		let y1 = ((p1.y + 1.0) * (1024.0 / 2.0)).floor() as i32;
		let x2 = ((p2.x + 1.0) * (1024.0 / 2.0)).floor() as i32;
		let y2 = ((p2.y + 1.0) * (1024.0 / 2.0)).floor() as i32;

		let dy = (y2 - y1) as f32;
		let dx = (x2 - x1) as f32;

		if dx.abs() > dy.abs()
		{
			let slope = dy / dx;
			let mut j = y1 as f32;
			for i in x1..x2
			{
				let j_rounded = j as i32;
				if i < 0 || i >= 1024 || j_rounded < 0 || j_rounded >= 1024
				{
					j = j + slope;
					continue;
				}

				pixels[(3 * 1024 * j_rounded + 3 * i + 0) as usize] = 255;
				pixels[(3 * 1024 * j_rounded + 3 * i + 1) as usize] = 255;
				pixels[(3 * 1024 * j_rounded + 3 * i + 2) as usize] = 255;
				j = j + slope;
			}
		}
		else if dx.abs() < dy.abs()
		{
			let slope = dx / dy;
			let mut i = x1 as f32;
			for j in y1..y2
			{
				let i_rounded = i as i32;
				if i_rounded < 0 || i_rounded >= 1024 || j < 0 || j >= 1024
				{
					i = i + slope;
					continue;
				}

				pixels[(3 * 1024 * j + 3 * i_rounded + 0) as usize] = 255;
				pixels[(3 * 1024 * j + 3 * i_rounded + 1) as usize] = 255;
				pixels[(3 * 1024 * j + 3 * i_rounded + 2) as usize] = 255;
				i = i + slope;
			}
		}
	}

	fn draw_triangle(p1: &na::Point3<f32>, p2: &na::Point3<f32>, p3: &na::Point3<f32>, pixels: &mut Vec<u8>, depth_buffer: &mut Vec<f32>)
	{

	}
}

fn main()
{
	// a cube whose origin is at (0,0,0), width = 0.5
	let front_top_left     = na::point!(-0.5f32, 0.5, 0.5);
	let front_top_right    = na::point!(0.5f32, 0.5, 0.5);
	let front_bottom_left  = na::point!(-0.5f32, -0.5, 0.5);
	let front_bottom_right = na::point!(0.5f32, -0.5, 0.5);
	let back_top_left      = na::point!(-0.5f32, 0.5, -0.5);
	let back_top_right     = na::point!(0.5f32, 0.5, -0.5);
	let back_bottom_left   = na::point!(-0.5f32, -0.5, -0.5);
	let back_bottom_right  = na::point!(0.5f32, -0.5, -0.5);

	let triangles = vec!
	[
		// Top
		Triangle::new(back_top_left, front_top_left, front_top_right),
		Triangle::new(back_top_left, back_top_right, front_top_right),

		// Bottom
		Triangle::new(back_bottom_left, front_bottom_left, front_bottom_right),
		Triangle::new(back_bottom_left, back_bottom_right, front_bottom_right),

		// Front
		Triangle::new(front_top_left, front_bottom_left, front_bottom_right),
		Triangle::new(front_top_left, front_top_right, front_bottom_right),

		// Back
		Triangle::new(back_top_left, back_bottom_left, back_bottom_right),
		Triangle::new(back_top_left, back_top_right, back_bottom_right),

		// Left side
		Triangle::new(front_top_left, front_bottom_left, back_bottom_left),
		Triangle::new(front_top_left, back_top_left, back_bottom_left),

		// Right side
		Triangle::new(front_top_right, front_bottom_right, back_bottom_right),
		Triangle::new(front_top_right, back_top_right, back_bottom_right)
	];

	let mut pixels : Vec<u8> = Vec::new();
	let mut depth_buffer : Vec<f32> = Vec::new();

	pixels.resize(1024 * 1024 * 3, 0);
	depth_buffer.resize(1024 * 1024, 1.0);

	let mut camera = Camera
	{
		position: na::Point3::new(0.0, 0.0, 1.1),
		rotation: na::UnitQuaternion::from_euler_angles(0.0, 0.0, 1.0),
		perspective: na::Perspective3::new(1024.0 / 1024.0, 60.0, 0.1, 20.0)
	};

	unsafe
	{
		let histance = GetModuleHandleA(None).unwrap();
		assert!(histance.0 != 0);
		let class_name =  s!("window");
		let wc = WNDCLASSA
		{
			//hCursor: LoadCursorA(None, None).unwrap(),
			hInstance: histance,
			lpszClassName: class_name,
			style: CS_HREDRAW | CS_VREDRAW,
			lpfnWndProc: Some(wndproc),
			..Default::default()
		};

		assert!(RegisterClassA(&wc) != 0);

		let hwnd = CreateWindowExA(WINDOW_EX_STYLE::default(), class_name, s!("Sample window"), WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT, CW_USEDEFAULT, 1024, 1024, None, None, histance, None);
		assert!(hwnd.0 != 0);

		let bitmap_info = BITMAPINFO
		{
			bmiHeader: BITMAPINFOHEADER
			{
				biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
				biWidth: 1024,
				biHeight: -1024,
				biPlanes: 1,
				biBitCount: 24,
				biCompression: BI_RGB,
				biSizeImage: 0,
				biXPelsPerMeter: 0,
				biYPelsPerMeter: 0,
				biClrUsed: 0,
				biClrImportant: 0
			},
			bmiColors: [RGBQUAD::default()]
		};
		let bits : *mut u8 = std::ptr::null_mut();
		let bits_address = &bits as *const *mut u8;

		let hbm_result = CreateDIBSection(None, &bitmap_info, DIB_RGB_COLORS, bits_address as *mut *mut std::ffi::c_void, None, 0);
		assert!(bits.is_null() == false);
		let hbm = hbm_result.unwrap();

		let bitmap_hdc = CreateCompatibleDC(None);
		assert!(bitmap_hdc.0 != 0);

		SelectObject(bitmap_hdc, hbm);

		let mut message = MSG::default();

		loop
		{
			while PeekMessageA(&mut message, HWND(0), 0, 0, PM_REMOVE).into()
			{
				TranslateMessage(&message);
				DispatchMessageA(&message);
			}

			let state = GetKeyState(87);
			if state > 0
			{
				camera.position.z = camera.position.z - 0.01;
			}

			for i in 0..1024*1024*3
			{
				pixels[i] = 0;
			}

			for triangle in &triangles
			{
				triangle.render(&camera, &mut pixels, &mut depth_buffer);
			}


			for j in 0..1024
			{
				for i in 0..1024
				{
					*(bits.offset(3 * 1024 * j + 3 * i + 0)) = pixels[3 * 1024 * j as usize + 3 * i as usize + 0];
					*(bits.offset(3 * 1024 * j + 3 * i + 1)) = pixels[3 * 1024 * j as usize + 3 * i as usize + 1];
					*(bits.offset(3 * 1024 * j + 3 * i + 2)) = pixels[3 * 1024 * j as usize + 3 * i as usize + 2];
				}
			}

			let hdc = GetDC(hwnd);
			assert!(hdc.0 != 0);
			assert!(BitBlt(hdc, 0, 0, 1024, 1024, bitmap_hdc, 0, 0, SRCCOPY) == true);
			assert!(ReleaseDC(hwnd, hdc) == 1);
		}
	}
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
	unsafe
	{
		return DefWindowProcA(window, message, wparam, lparam);
	}
}
