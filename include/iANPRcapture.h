/******************************************************************************
	(C) 2018. IntBuSoft. http://intbusoft.com
	iANPRcapture.h - version 1.7
*******************************************************************************/
#pragma once
#include "iANPR.h"
#include <opencv2/imgproc/imgproc.hpp>

typedef void* iANPRCapture;

/*
	Создание потокового распознавания.
	max_frames - количество кадров в  буфере;
	Options - настройки распознавания;
	Rect - область распознавания.
	Функция возвращает указатель на потоковый объект
*/
iANPRCapture 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
CreateiANPRCapture( int max_frames, ANPR_OPTIONS Options, CvRect Rect );

// Очистка памяти для потокового распознавания
void 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 	
ReleaseiANPRCapture( iANPRCapture *Capture );

/*
	Добавление кадра (распознавание).
	Capture - созданные ранее объект потокового распознавания;
	Image - очередное изображение (8 - битное цветное (3 канала) или 8 битное 1 канальное);
	AllNumber - возвращается количество распознанных номеров на последний кадр;
	Texts - содержание номера.
	Функция возвращает 0 при успешном вызове, иначе ошибка.
*/
int 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 	
AddFrameToiANPRCapture( iANPRCapture Capture, IplImage* Image, int* AllNumber, CvRect* Rects, char** Texts );

int 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 	
AddFrameToiANPRCaptureMat( iANPRCapture Capture, cv::Mat Mat, int* AllNumber, CvRect* Rects, char** Texts );

/* 
	Создание памяти объединенного результата
	Capture - созданный ранее объект потокового распознавания; 
	min_frames_with_plate - количество кадров между первым и последним распознанным номером, после которого можно считать, что это действительно номер;
	frames_without_plate - количество кадров без распознанного ранее номера, после которого можно выдавать результат; 
	max_plates_in_mem - максимальное количество номеров в памяти.
	Функция возвращает 0 при успешном вызове, иначе ошибка.
*/
int 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 	
CreateMemoryForiANPRCapture( iANPRCapture Capture, int min_frames_with_plate, int frames_without_plate, int max_plates_in_mem );

/*
	Возвращает суммированный результат нахождения номера из памяти. Он возвращается только после frames_without_plate кадров.
	Capture - созданный ранее объект потокового распознавания; 
	AllNumber - возвращается количество распознанных номеров на последний кадр из памяти;
	Texts - содержание номера;
	Size_Texts - размер каждого массива Texts;
	Points - указатель на выделенный пред вызовом массив CvPoint для траектории (если NULL, то не возвращается);
	all_points - сюда передается размер массива Points, а возвращается количество найденных точек, если массив меньше (рекомендуется 1000 элементов), чем
	реально найдено точек, то будет возвращено только то количество точек, для которых есть место.
	Функция возвращает 0 при успешном вызове, иначе ошибка.

	ВНИМАНИЕ! Если возвращаются одновременно 2 номера в кадре, что маловероятно, по причине суммирования и отсрочки, то будет возвращатьс траектория 
	только для первого.
*/
int 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 	
GetNumbersInMemory( iANPRCapture Capture,  int* AllNumber, 
					char** Texts, int Size_Texts,
					CvPoint* Points, int* all_point );

/*
	Функция создает линию длдя фиксации пересечения. На самом деле линия состоит из двух линий -
	и пересечение фиксируется только тогда, когда пересекаются обе (сделано для исключения ложных
	срабатываний. 
	Capture - созданный ранее объект потокового распознавания; 
	p1a и p2a - точки характеризующие верхнюю линию (линия не может быть вертикальной - максимум
	дельта x может быть в 3 раза меньше дельта y. Иначе ошибка ERROR_SLOPE_LINE); 
	p1b и p2b - точки характеризующие нижнюю линию.
	Функция возвращает 0 при успешном вызове, иначе ошибка.

	ВНИМАНИЕ! Линии должны быть параллельны друг другу. Иначе ERROR_NO_PARALLEL_LINES.
*/
int 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 	
CreateLineIntersection( iANPRCapture Capture, CvPoint p1a, CvPoint p2a, 
						CvPoint p1b, CvPoint p2b );


void 
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
LicenseCapture( char* lic );