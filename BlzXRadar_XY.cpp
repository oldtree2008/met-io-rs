// BlzXRadar_XY.cpp: 定义应用程序的入口点。
//

#include "BlzXRadar_XY.h"
#include <math.h>
using namespace std;

BlzXRadar_XY::BlzXRadar_XY(const char* fileName)
{

	this->_fileName = new(std::nothrow)char[1024];//为文件名称开辟空间
	strcpy(this->_fileName, fileName);
	//判断是不是被压缩的
	int len = strlen(fileName);
	//现在只考虑bz2压缩格式
	if (this->_fileName[len - 3] == 'b' && this->_fileName[len - 2] == 'z' && this->_fileName[len - 1] == '2')
	{
		_isZip = true;
		_zipType = 1;
	}
	else if (this->_fileName[len - 3] == 'z' && this->_fileName[len - 2] == 'i' && this->_fileName[len - 1] == 'p')
	{
		_isZip = true;
		_zipType = 2;
	}
	else {
		_isZip = false;
		_zipType = 0;
	}
	_readSize = 0;

	_elNum = 0;
	_azNum = 0;
	_refBinNum = 0;
	_dplBinNum = 0;

	_time = NULL;
	this->_volHead = NULL;
	this->_elList = NULL;	//仰角
	this->_refAzList = NULL;//反射率方位角
	this->_dplAzList = NULL;//多普勒方位角

	this->_refList = NULL;	//反射率
	this->_velList = NULL;	//速度
	this->_spwList = NULL;	//谱宽
	this->_zdrList = NULL;	//差分反射率因子
	this->_phiList = NULL;	//差分相位
	this->_kdpList = NULL;	//差分相移
	this->_rhvList = NULL;	//零延迟相关系数

	this->_Data = NULL;		//存储解压后数据 双偏振 100M
	this->_Dptr = NULL;		//指向当前要读数据的位置


	_RefAzAlignList = NULL;
	_DplAzAlignList = NULL;

	_RefAlignList = NULL;
	_VelAlignList = NULL;
	_SwAlignList = NULL;


	_ZdrAlignList = NULL;
	_PhiAlignList = NULL;
	_KdpAlignList = NULL;
	_RhvAlignList = NULL;
	
}

BlzXRadar_XY::~BlzXRadar_XY()
{
	close();
}

int BlzXRadar_XY::ReadData()
{
	ReadFileData();
	GetVolHead();
	GetBinsLens();
	GetAzData();
	GetProductData();
	GetTime();
	//Align();
	return 0;
}

void BlzXRadar_XY::GetRadarLat(float* fnum)
{
	*fnum = this->_volHead->AddressP.Latitude / 360000.;
}

void BlzXRadar_XY::GetRadarLon(float* fnum)
{
	*fnum = this->_volHead->AddressP.Longitude / 360000.;
}

void BlzXRadar_XY::GetRadarAlt(float* fnum)
{
	*fnum = this->_volHead->AddressP.Height / 1000000.;
}

void BlzXRadar_XY::GetElNum(short* snum)
{
	*snum = this->_elNum;
}

void BlzXRadar_XY::GetAzNum(short* snum)
{
	*snum = ALIGENRADS;
}

void BlzXRadar_XY::GetRefBinNum(short* snum)
{
	*snum = this->_refBinNum;
}

void BlzXRadar_XY::GetDplBinNum(short* snum)
{
	*snum = this->_dplBinNum;
}

void BlzXRadar_XY::GetRefBinLen(float* fnum)
{ 
	*fnum = this->_refBinLen;
}

void BlzXRadar_XY::GetDplBinLen(float* fnum)
{
	*fnum = this->_dplBinLen;
}

int BlzXRadar_XY::GetElList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum, "ElList arr no enough!\n");
	for (int i = 0; i < _elNum; i++)
	{
		fdata[i] = this->_volHead->SurveyP.elevation[i] / 100.;
	}
	return 0;
}

int BlzXRadar_XY::GetRefAzList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS, "RefAzList arr no enough!\n");
	for (int el = 0; el < _elNum; el++)
	{
		for (int az = 0; az < ALIGENRADS; az++)
		{
			fdata[el * ALIGENRADS + az] = this->_refAzList[el * ALIGENRADS + az];
		}
	}
	return 0;
}

int BlzXRadar_XY::GetDplAzList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS, "DplAzList arr no enough!\n");
	for (int el = 0; el < _elNum; el++)
	{
		for (int az = 0; az < ALIGENRADS; az++)
		{
			fdata[el * ALIGENRADS + az] = this->_dplAzList[el * ALIGENRADS + az];
		}
	}
	return 0;
}

int BlzXRadar_XY::GetRefList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS * _refBinNum, "RefList length not enough!\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bin = 0; bin < _refBinNum; bin++)
			{
				fdata[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_refList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin];
			}
	return 0;
}

int BlzXRadar_XY::GetVelList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS * _dplBinNum, "VelList length not enough!\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bin = 0; bin < _dplBinNum; bin++)
			{
				fdata[el * ALIGENRADS * _dplBinNum + az * _dplBinNum + bin] = this->_velList[el * ALIGENRADS * _dplBinNum + az * _dplBinNum + bin];
			}
	return 0;
}

int BlzXRadar_XY::GetSwList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS * _refBinNum, "SwList length not enough!\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bin = 0; bin < _refBinNum; bin++)
			{
				fdata[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_spwList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin];
			}
	return 0;
}

int BlzXRadar_XY::GetZdrList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS * _refBinNum, "ZdrList length not enough!\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bin = 0; bin < _refBinNum; bin++)
			{
				fdata[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_zdrList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin];
			}
	return 0;
}

int BlzXRadar_XY::GetPhiList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS * _refBinNum, "PhiList length not enough!\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bin = 0; bin < _refBinNum; bin++)
			{
				fdata[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_phiList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin];
			}
	return 0;
}

int BlzXRadar_XY::GetKdpList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS * _refBinNum, "KdpList length not enough!\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bin = 0; bin < _refBinNum; bin++)
			{
				fdata[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_kdpList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin];
			}
	return 0;
}

int BlzXRadar_XY::GetRhvList(float* fdata, int len)
{
	ARR_LEN_ERROR(len, _elNum * ALIGENRADS * _refBinNum, "RhvList length not enough!\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bin = 0; bin < _refBinNum; bin++)
			{
				fdata[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_rhvList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin];
			}
	return 0;
}

int BlzXRadar_XY::CGetRefAzList(float** fdata, int ElNum, int AzNum)
{
	ARR_LEN_ERROR(ElNum * AzNum, this->_elNum * ALIGENRADS, "The refaz length of the incoming parameter is wrong !\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			fdata[el][az] = _refAzList[el * ALIGENRADS + az];
	return 0;
}

int BlzXRadar_XY::CGetDplAzList(float** fdata, int ElNum, int AzNum)
{
	ARR_LEN_ERROR(ElNum * AzNum, this->_elNum * ALIGENRADS, "The refaz length of the incoming parameter is wrong !\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			fdata[el][az] = _dplAzList[el * ALIGENRADS + az];
	return 0;
}

int BlzXRadar_XY::CGetRefList(float*** fdata, int ElNum, int AzNum, int BinNum)
{
	ARR_LEN_ERROR(ElNum * AzNum * BinNum, _elNum * _refBinNum * ALIGENRADS, "The ref length of the incoming parameter is wrong !\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bn = 0; bn < _refBinNum; bn++)
				fdata[el][az][bn] = _refList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bn];
	return 0;
}

int BlzXRadar_XY::CGetVelList(float*** fdata, int ElNum, int AzNum, int BinNum)
{
	ARR_LEN_ERROR(ElNum * AzNum * BinNum, _elNum * _refBinNum * ALIGENRADS, "The ref length of the incoming parameter is wrong !\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bn = 0; bn < _refBinNum; bn++)
				fdata[el][az][bn] = _velList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bn];
	return 0;
}

int BlzXRadar_XY::CGetSwList(float*** fdata, int ElNum, int AzNum, int BinNum)
{
	ARR_LEN_ERROR(ElNum * AzNum * BinNum, _elNum * _refBinNum * ALIGENRADS, "The ref length of the incoming parameter is wrong !\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bn = 0; bn < _refBinNum; bn++)
				fdata[el][az][bn] = _spwList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bn];
	return 0;
}

int BlzXRadar_XY::CGetZdrList(float*** fdata, int ElNum, int AzNum, int BinNum)
{
	ARR_LEN_ERROR(ElNum * AzNum * BinNum, _elNum * _refBinNum * ALIGENRADS, "The ref length of the incoming parameter is wrong !\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bn = 0; bn < _refBinNum; bn++)
				fdata[el][az][bn] = _zdrList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bn];
	return 0;
}

int BlzXRadar_XY::CGetPhiList(float*** fdata, int ElNum, int AzNum, int BinNum)
{
	ARR_LEN_ERROR(ElNum * AzNum * BinNum, _elNum * _refBinNum * ALIGENRADS, "The ref length of the incoming parameter is wrong !\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bn = 0; bn < _refBinNum; bn++)
				fdata[el][az][bn] = _phiList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bn];
	return 0;
}

int BlzXRadar_XY::CGetKdpList(float*** fdata, int ElNum, int AzNum, int BinNum)
{
	ARR_LEN_ERROR(ElNum * AzNum * BinNum, _elNum * _refBinNum * ALIGENRADS, "The ref length of the incoming parameter is wrong !\n");
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bn = 0; bn < _refBinNum; bn++)
				fdata[el][az][bn] = _kdpList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bn];
	return 0;
}

int BlzXRadar_XY::CGetRhvList(float*** fdata, int ElNum, int AzNum, int BinNum)
{
	for (int el = 0; el < _elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
			for (int bn = 0; bn < _refBinNum; bn++)
				fdata[el][az][bn] = _rhvList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bn];
	return 0;
}

void BlzXRadar_XY::GetTimeStamp(char* ctime)
{
	strcpy(ctime, _time);
}

int BlzXRadar_XY::GetTime()
{
	ALL_MEM_CHAR(_time, 24, "time malloc error!\n");
	time_t t;
	sprintf(_time, "%d-%02d-%02d %02d:%02d:%02d", this->_volHead->SurveyP.EndYear, this->_volHead->SurveyP.EndMouth, this->_volHead->SurveyP.EndDay, this->_volHead->SurveyP.EndHour, this->_volHead->SurveyP.EndMinute, this->_volHead->SurveyP.EndSecond);
	int num = GetTick(_time);
	delete[]_time;
	_time = NULL;
	ALL_MEM_CHAR(_time, 24, "time malloc error!\n");
	time_t t1;
	t1 = num;
	struct tm* lt;
	lt = localtime(&t1);
	memset(_time, 0, sizeof(_time));
	strftime(_time, 24, "%Y%m%d%H%M%S", lt);

	return 0;
}

long BlzXRadar_XY::GetTick(char* str_time)
{
	struct tm stm;
	int iY, iM, iD, iH, iMin, iS;
	memset(&stm, 0, sizeof(stm));
	iY = atoi(str_time);
	iM = atoi(str_time + 5);
	iD = atoi(str_time + 8);
	iH = atoi(str_time + 11);
	iMin = atoi(str_time + 14);
	iS = atoi(str_time + 17);
	stm.tm_year = iY - 1900;
	stm.tm_mon = iM - 1;
	stm.tm_mday = iD;
	stm.tm_hour = iH;
	stm.tm_min = iMin;
	stm.tm_sec = iS;
	return mktime(&stm);
}

void BlzXRadar_XY::close()
{
	DestoryMemory();
}

void BlzXRadar_XY::DestoryMemory()
{
	DES_MEM(_fileName);
	DES_MEM(_Data);
	DES_MEM(_elList);
	DES_MEM(_refAzList);
	DES_MEM(_dplAzList);
	DES_MEM(_refList);
	DES_MEM(_velList);
	DES_MEM(_spwList);
	DES_MEM(_zdrList);
	DES_MEM(_phiList);
	DES_MEM(_kdpList);
	DES_MEM(_rhvList);

	DES_MEM(_RefAzAlignList);
	DES_MEM(_DplAzAlignList);
	DES_MEM(_RefAlignList);
	DES_MEM(_VelAlignList);
	DES_MEM(_SwAlignList);
	DES_MEM(_ZdrAlignList);
	DES_MEM(_PhiAlignList);
	DES_MEM(_KdpAlignList);
	DES_MEM(_RhvAlignList);
	
}

int BlzXRadar_XY::InitFileData()
{
	if (!_Data)
	{
		//首先判断是不是被压缩的，如果没有被压缩，直接申请同等大小空间
		if (this->_isZip)
		{
			//被压缩就直接申请120M空间   双偏振一般预留100M
			ALLOT_MEM_CHAR(_Data, 120 * 1024 * 1024, "Data malloc err!\n");
		}
		else
		{
			int len = GetFileSzie(_fileName);
			ALLOT_MEM_CHAR(_Data, len, "Data malloc err!\n");
		}
	}
	return 0;
}

int BlzXRadar_XY::ReadFileData()
{
	MEM_ERROR(this->_fileName, "equest space failed!\n");
	OPEN_FILE_ERROR(this->_fileName, "in read funtion fileName is NULL!\n");

	BZFILE* bzFile = NULL;
	int bzerror = 0;         //bz2 解压标识符

	this->InitFileData();//初始化资源
	if (this->_zipType ==1 )//修改为 == 1
	{
		int bzerror = 0;
		BZFILE* bzFile = BZ2_bzopen(_fileName, "rb");
		NULL_ERROR(_fileName, "the filename is empty\n");
		OPEN_FILE_ERROR(bzFile, "bzFile is failed\n");
		this->_readSize = BZ2_bzRead(&bzerror, bzFile, this->_Data, 120 * 1024 * 1024); 
		BZ2_bzclose(bzFile);
		READ_FILE_ERROR(this->_readSize, "readSize is smaller\n");
	}
	else if (this->_zipType == 2) //添加zip解压模式
	{
		unzFile zFile;
		zFile = unzOpen64(_fileName);
		unzOpenCurrentFile(zFile);
		NULL_ERROR(_fileName, "the filename is empty\n");
		OPEN_FILE_ERROR(zFile, "bzFile is failed\n");
		this->_readSize = unzReadCurrentFile(zFile, this->_Data, 120 * 1024 * 1024);
		unzCloseCurrentFile(zFile);
		READ_FILE_ERROR(this->_readSize, "readSize is smaller\n");
	}
	else
	{
		FILE* fp = fopen(_fileName, "rb");
		MEM_ERROR(fp, "open file is failed");
		int len = this->GetFileSzie(_fileName);
		_readSize = fread(this->_Data, len, 1, fp);
		_readSize *= len;
		fclose(fp);
		READ_FILE_ERROR(this->_readSize, "readSize is smaller\n");
	}
	return 0;
}

int BlzXRadar_XY::InitVolHead()
{
	ALLOT_MEM_HEAD(this->_volHead, VOLFILEHEADER, "VolHead malloc is err!\n");
	return 0;
}

void BlzXRadar_XY::GetVolHead()
{
	if (!this->_volHead)
	{
		this->InitVolHead();
	}
	this->_Dptr = this->_Data;
	memcpy(this->_volHead, this->_Dptr, sizeof(VOLFILEHEADER));
}

int BlzXRadar_XY::InitAzData()
{	
	ALLOT_MEM_FLOAT(_elList, _elNum, "ElList malloc is err!\n");
	ALLOT_MEM_FLOAT(_refAzList, _elNum * _azNum, "RefAzList malloc is err!\n");
	ALLOT_MEM_FLOAT(_dplAzList, _elNum * _azNum, "DplAzList malloc is err!\n");
	ALLOT_MEM_FLOAT(_RefAzAlignList, _elNum * ALIGENRADS, "DplAzList malloc is err!\n");
	ALLOT_MEM_FLOAT(_DplAzAlignList, _elNum * ALIGENRADS, "DplAzList malloc is err!\n");
	return 0;
}

void BlzXRadar_XY::GetAzData()
{
	this->InitAzData();
	RADIOHEAD tmp = { 0 };
	for (int el = 0; el < _elNum; el++)
	{
		//定位数据指针
		int refBins = this->_volHead->SurveyP.ERDistanceNum[el];
		int dplBins = this->_volHead->SurveyP.BPM.ECDistanceNum[el];
		int lenght = sizeof(RADIOHEAD) + refBins * 7 + dplBins * 2;
		int offset = this->_volHead->SurveyP.PPIInFileSD[el];
		int az_num = this->_volHead->SurveyP.ERadialNum[el];
		this->_Dptr = this->_Data + offset;
		for (int az = 0; az < az_num; az++)
		{

			unsigned char* tmp_ptr = this->_Dptr + (az * lenght);
			memcpy(&tmp, tmp_ptr, sizeof(RADIOHEAD));
			int tmp_i = tmp.Az;
			this->_refAzList[el * _azNum + az] = tmp.Az / 100.;
			this->_dplAzList[el * _azNum + az] = tmp.Az / 100.;
		}
	}
}

int BlzXRadar_XY::InitProductData()
{
	ALLOT_MEM_FLOAT(_refList, _elNum * _azNum * _refBinNum, "_refList malloc is err!\n");
	ALLOT_MEM_FLOAT(_velList, _elNum * _azNum * _dplBinNum, "_velList malloc is err!\n");
	ALLOT_MEM_FLOAT(_spwList, _elNum * _azNum * _dplBinNum, "_spwList malloc is err!\n");
	ALLOT_MEM_FLOAT(_zdrList, _elNum * _azNum * _refBinNum, "_zdrList malloc is err!\n");
	ALLOT_MEM_FLOAT(_phiList, _elNum * _azNum * _refBinNum, "_phiList malloc is err!\n");
	ALLOT_MEM_FLOAT(_kdpList, _elNum * _azNum * _refBinNum, "_kdpList malloc is err!\n");
	ALLOT_MEM_FLOAT(_rhvList, _elNum * _azNum * _refBinNum, "_rhvList malloc is err!\n");

	ALLOT_MEM_FLOAT(_RefAlignList, _elNum * ALIGENRADS * _refBinNum, "_RefAlignList malloc is err!\n");
	ALLOT_MEM_FLOAT(_VelAlignList, _elNum * ALIGENRADS * _dplBinNum, "_VelAlignList malloc is err!\n");
	ALLOT_MEM_FLOAT(_SwAlignList,  _elNum * ALIGENRADS * _dplBinNum, "_SwAlignList malloc is err!\n");
	ALLOT_MEM_FLOAT(_ZdrAlignList, _elNum * ALIGENRADS * _refBinNum, "_ZdrAlignList malloc is err!\n");
	ALLOT_MEM_FLOAT(_PhiAlignList, _elNum * ALIGENRADS * _refBinNum, "_PhiAlignList malloc is err!\n");
	ALLOT_MEM_FLOAT(_KdpAlignList, _elNum * ALIGENRADS * _refBinNum, "_KdpAlignList malloc is err!\n");
	ALLOT_MEM_FLOAT(_RhvAlignList, _elNum * ALIGENRADS * _refBinNum, "_RhvAlignList malloc is err!\n");
	return 0;
}

void BlzXRadar_XY::GetProductData()
{
	this->InitProductData();
	for (int el = 0; el < _elNum; el++)
	{
		//定位数据指针
		int refBins = this->_volHead->SurveyP.ERDistanceNum[el];
		int dplBins = this->_volHead->SurveyP.BPM.ECDistanceNum[el];
		int lenght = sizeof(RADIOHEAD) + refBins * 7 + dplBins * 2;
		int offset = this->_volHead->SurveyP.PPIInFileSD[el];
		int az_num = this->_volHead->SurveyP.ERadialNum[el];

		this->_Dptr = this->_Data + offset;
		for (int az = 0; az < az_num; az++)
		{
			//定位到数据位置
			unsigned char* tmp_ptr = this->_Dptr + (az * lenght) + sizeof(RADIOHEAD);
			//提取反射率 一个字节
			for (int bin = 0; bin < refBins; bin++)
			{
				short value = tmp_ptr[bin];
				this->_refList[el * _azNum * _refBinNum + az * _refBinNum + bin] = DecodeRef(value);
			}
			//提取速度 一个字节
			//定位数据位置
			tmp_ptr = this->_Dptr + (az * lenght) + sizeof(RADIOHEAD) + refBins;
			for (int bin = 0; bin < dplBins; bin++)
			{
				short value = tmp_ptr[bin];

				this->_velList[el * _azNum * _dplBinNum + az * _dplBinNum + bin] = DecodeVel(value);
			}
			//提取谱宽一个字节
			//定位数据位置
			tmp_ptr = this->_Dptr + (az * lenght) + sizeof(RADIOHEAD) + refBins + dplBins;
			for (int bin = 0; bin < dplBins; bin++)
			{
				short value = tmp_ptr[bin];
				this->_spwList[el * _azNum * _dplBinNum + az * _dplBinNum + bin] = DecodeSw(value);
			}

			//提取差分反射率因子，一个字节
			//定位数据位置
			tmp_ptr = this->_Dptr + (az * lenght) + sizeof(RADIOHEAD) + refBins * 2 + dplBins * 2;
			for (int bin = 0; bin < refBins; bin++)
			{
				short value = tmp_ptr[bin];
				this->_zdrList[el * _azNum * _refBinNum + az * _refBinNum + bin] = DecodeZdr(value);
			}
			//提取零延迟相关系数，一个字节
			//定位数据位置
			tmp_ptr = this->_Dptr + (az * lenght) + sizeof(RADIOHEAD) + refBins * 3 + dplBins * 2;
			for (int bin = 0; bin < refBins; bin++)
			{
				short value = tmp_ptr[bin];
				this->_rhvList[el * _azNum * _refBinNum + az * _refBinNum + bin] = DecodeRhv(value);
			}
			//提取差分相移，一个字节
			//定位数据位置
			tmp_ptr = this->_Dptr + (az * lenght) + sizeof(RADIOHEAD) + refBins * 4 + dplBins * 2;
			for (int bin = 0; bin < refBins; bin++)
			{
				short value = tmp_ptr[bin];
				this->_kdpList[el * _azNum * _refBinNum + az * _refBinNum + bin] = DecodeKdp(value);
			}
			//提取差分相位，两个字节
			//定位数据位置
			tmp_ptr = this->_Dptr + (az * lenght) + sizeof(RADIOHEAD) + refBins * 5 + dplBins * 2;
			for (int bin = 0; bin < refBins; bin++)
			{
				unsigned short value = ((unsigned short*)tmp_ptr)[bin];
				this->_phiList[el * _azNum * _refBinNum + az * _refBinNum + bin] = DecodePhi(value);
			}
		}
	}
}

inline float BlzXRadar_XY::DecodeRef(short value)
{
	float res;
	if (value < 2 || value>255)
		res = INVALIDVALUE;
	else {
		res = value * 0.5 - 33;
	}
	return res;
}

inline float BlzXRadar_XY::DecodeVel(short value)
{
	float res;
	if (value < 2 || value>255)
		res = INVALIDVALUE;
	else {
		res = value * 0.5 - 64.5;
	}
	return res;
}

inline float BlzXRadar_XY::DecodeSw(short value)
{
	float res;
	if (value < 129 || value>255)
		res = INVALIDVALUE;
	else {
		res = value * 0.5 - 64.5;
	}
	return res;
}

inline float BlzXRadar_XY::DecodeZdr(short value)
{
	float res;
	if (value < 20 || value>110)
		res = INVALIDVALUE;
	else {
		res = value * 0.1 - 5;
	}
	return res;
}

inline float BlzXRadar_XY::DecodeRhv(short value)
{
	float res;
	if (value < 5 || value>105)
		res = INVALIDVALUE;
	else {
		res = value * 0.01 - 0.05;
	//	if (res < 0)res = -res;
	}
	return res;
}

inline float BlzXRadar_XY::DecodeKdp(short value)
{
	float res;
	if (value < 20 || value>160)
		res = INVALIDVALUE;
	else {
		res = value * 0.05 - 3;
	}
	return res;
}
inline float BlzXRadar_XY::DecodePhi(unsigned short value)
{
	float res;
	if (value == 0 || value == 1)
		res = INVALIDVALUE;
	if (value == 2)res = 0.000;
	if (value == 65535) res = 359.995;
	else {
		res = 360 * (value - 2) / 65534.;
	}
	return res;
}
int BlzXRadar_XY::GetFileSzie(char* fileName)
{
	FILE* fp = fopen(fileName, "rb");
	fseek(fp, 0, SEEK_END); //定位到文件末 
	int nFileLen = ftell(fp); //文件长度
	fclose(fp);
	return nFileLen;
}
void BlzXRadar_XY::GetBinsLens()
{
	_elNum = this->_volHead->SurveyP.SSLayerNumber;
	_azNum = this->_volHead->SurveyP.ERadialNum[0];
	_refBinNum = this->_volHead->SurveyP.ERDistanceNum[0];
	_dplBinNum = this->_volHead->SurveyP.BPM.ECDistanceNum[0];
	_refBinLen = this->_volHead->SurveyP.ERlibL[0] / 1000.;
	_dplBinLen = this->_volHead->SurveyP.EClutterL[0] / 1000.;
}

int BlzXRadar_XY::Align()
{
	for (int el = 0; el < this->_elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
		{
			int var = GetIndexofAz(az, _azNum, _refAzList + el * _azNum);
			for (int bin = 0; bin < _refBinNum; bin++)
			{
				this->_RefAlignList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_refList[el * _azNum * _refBinNum + var * _refBinNum + bin];
				this->_ZdrAlignList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_zdrList[el * _azNum * _refBinNum + var * _refBinNum + bin];
				this->_PhiAlignList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_phiList[el * _azNum * _refBinNum + var * _refBinNum + bin];
				this->_KdpAlignList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_kdpList[el * _azNum * _refBinNum + var * _refBinNum + bin];
				this->_RhvAlignList[el * ALIGENRADS * _refBinNum + az * _refBinNum + bin] = this->_rhvList[el * _azNum * _refBinNum + var * _refBinNum + bin];
			}
			*(_RefAzAlignList + el * ALIGENRADS + az) = az;
		}
	DES_MEM_ALIGN(_refAzList, _RefAzAlignList);
	DES_MEM_ALIGN(_refList, _RefAlignList);
	DES_MEM_ALIGN(_zdrList, _ZdrAlignList);
	DES_MEM_ALIGN(_phiList, _PhiAlignList);
	DES_MEM_ALIGN(_kdpList, _KdpAlignList);
	DES_MEM_ALIGN(_rhvList, _RhvAlignList);
	for (int el = 0; el < this->_elNum; el++)
		for (int az = 0; az < ALIGENRADS; az++)
		{
			int var = GetIndexofAz(az, _azNum, _dplAzList + el * _azNum);
			for (int bin = 0; bin < _dplBinNum; bin++)
			{
				this->_VelAlignList[el * ALIGENRADS * _dplBinNum + az * _dplBinNum + bin] = this->_velList[el * _azNum * _dplBinNum + var * _dplBinNum + bin];
				this->_SwAlignList[el * ALIGENRADS * _dplBinNum + az * _dplBinNum + bin] = this->_spwList[el * _azNum * _dplBinNum + var * _dplBinNum + bin];
			}
			*(_DplAzAlignList + el * ALIGENRADS + az) = az;
		}
	DES_MEM_ALIGN(_dplAzList, _DplAzAlignList);
	DES_MEM_ALIGN(_velList, _VelAlignList);
	DES_MEM_ALIGN(_spwList, _SwAlignList);

	return 0;
}

int BlzXRadar_XY::GetIndexofAz(unsigned short nAzIndex, unsigned short nAzNum, float* fAzArray)
{

	int i;
	float fAz, fAz1;
	int azTmp = 0;
	float aztemp;
	float fAzIndex = (float)nAzIndex;
	for (i = 0; i < nAzNum - 1; i++)
	{

		if (modf(fAzArray[i], &aztemp) > 0.5)azTmp = (float)(aztemp + 1);
		else azTmp = (float)aztemp;
		if (nAzIndex == azTmp)return i;
	}
	for (i = 0; i < nAzNum - 1; i++)
	{

		fAz = fAzArray[i];
		fAz1 = fAzArray[i + 1];

		if (fAz < fAz1)
		{
			if ((fAz <= fAzIndex) && (fAz1 >= fAzIndex))
			{
				if (fabs(fAzIndex - fAz) > fabs(fAz1 - fAzIndex))
					return i + 1;
				else
					return i;
			}

		}
		else
		{
			fAz1 += 360.0;

			if ((fAz <= fAzIndex) && (fAz1 >= fAzIndex))
			{
				if (fabs(fAzIndex - fAz) > fabs(fAz1 - fAzIndex))
					return i + 1;
				else
					return i;
			}
			if ((fAzIndex < fAz) && (fAzIndex < fAz1 - 360.0))
			{
				fAzIndex += 360.0;
				if (fabs(fAzIndex - fAz) > fabs(fAz1 - fAzIndex))
					return i + 1;
				else
					return i;
			}
		}

	}
	return 0;
}
