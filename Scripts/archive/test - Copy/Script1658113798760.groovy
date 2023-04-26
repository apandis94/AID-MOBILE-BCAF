import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

request_search = WS.sendRequest(findTestObject('AID-Transaksi/1. PPKD - Search Realisasi', [('moid') : '20000012']))

//store json response to variable
def slurper_search = new groovy.json.JsonSlurper()

def parsedJson_count = slurper_search.parseText(request_search.getResponseText())

// count array size
def expectedSize = parsedJson_count.size()

def search = slurper_search.parseText(request_search.getResponseBodyContent())

KeywordUtil.logInfo("$search")

// get var in console
KeywordUtil.logInfo('jumlah is :' + expectedSize)

// get spesific array
def spesific = search.findAll({ Map map ->
        map.get('DealerName').contains('Asia Berjaya Mobilindo')
    })

println(spesific)

//  ^^^^^^^^^^^^^^^^^
def accno = spesific[0].NoKontrak

println(accno)

// store norek
String norek = accno.substring(0, 10)

// store nopin
String nopin = accno.substring(11, 14)