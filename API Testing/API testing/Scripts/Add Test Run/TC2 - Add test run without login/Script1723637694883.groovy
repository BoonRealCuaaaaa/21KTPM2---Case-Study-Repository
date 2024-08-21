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

res2 = WS.sendRequest(findTestObject('Add Test Run', [('cookie') : '', ('testcases') : ['207'], ('title') : 'Title', ('desc') : 'Desc'
            , ('env') : 'not-set', ('tag') : 'Manual', ('type') : 'Manual']))

WS.verifyResponseStatusCode(res2, 302)

Map<String, List> headers = res2.getHeaderFields()

// Lấy giá trị của tiêu đề 'Location'
String redirectUrl = (headers['Location']).get(0)

println('Redirect URL: ' + redirectUrl)

// Xác minh URL chuyển hướng
WS.verifyEqual(redirectUrl, '/login')

