/******************************************************************************
	(C) 2018. IntBuSoft. http://intbusoft.com
	iANPRinterface.h - version 1.7
*******************************************************************************/
#pragma once
#include "iANPR.h"
#include <opencv2/imgproc/imgproc.hpp>

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateMemory( char* in_buffer, int size_buffer, ANPR_OPTIONS Options, int* AllNumber, CvRect* Rects, char** Texts );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateMemoryRect( char* in_buffer, int size_buffer, CvRect Rect, ANPR_OPTIONS Options, int* AllNumber, CvRect* Rects, char** Texts );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateMat( cv::Mat Image, ANPR_OPTIONS Options, int* AllNumber, CvRect* Rects, char** Texts );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateMatRect( cv::Mat Image, CvRect Rect, ANPR_OPTIONS Options, int* AllNumber, CvRect* Rects, char** Texts );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateXML( IplImage* Image8, ANPR_OPTIONS Options, char* xml_buffer, int* size_xml_buffer );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateRectXML( IplImage* Image8, CvRect Rect, ANPR_OPTIONS Options, char* xml_buffer, int* size_xml_buffer );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateMemoryXML( char* in_buffer, int size_buffer, ANPR_OPTIONS Options, char* xml_buffer, int* size_xml_buffer );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateMemoryRectXML( char* in_buffer, int size_buffer, CvRect Rect, ANPR_OPTIONS Options, char* xml_buffer, int* size_xml_buffer );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateMatXML( cv::Mat Image, ANPR_OPTIONS Options, char* xml_buffer, int* size_xml_buffer );

int
#ifdef WIN32
__stdcall 
#else
__attribute__ ((visibility("default")))
#endif 
anprPlateMatRectXML( cv::Mat Image, CvRect Rect, ANPR_OPTIONS Options, char* xml_buffer, int* size_xml_buffer );