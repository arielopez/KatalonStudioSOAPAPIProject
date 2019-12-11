<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetCurrency</name>
   <tag></tag>
   <elementGuidId>48791ad1-e3f2-4072-aec1-9c8b40338718</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;CountryCurrency xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;>
            &lt;sCountryISOCode>${countryISOCode}&lt;/sCountryISOCode>
        &lt;/CountryCurrency>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>CountryCurrency</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.countryISOCode</defaultValue>
      <description></description>
      <id>5a50c583-136b-4c39-bfc7-6a57480754fb</id>
      <masked>false</masked>
      <name>countryISOCode</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementText(response, 'ListOfCurrenciesByNameResponse.ListOfCurrenciesByNameResult.tCurrency[0].sISOCode', 'AFA')
//WS.verifyElementText(response, 'ListOfCurrenciesByNameResponse.ListOfCurrenciesByNameResult.tCurrency[2].sName', 'MGA')
//WS.verifyElementText(response, 'ListOfCurrenciesByNameResponse.ListOfCurrenciesByNameResult.tCurrency[2].sName', 'MGA')
//WS.verifyElementText(response, 'ListOfCurrenciesByNameResponse.ListOfCurrenciesByNameResult.tCurrency[0].sISOCode', 'AFA')


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
//WS.verifyElementText(response, 'CountryCurrencyResponse.CountryCurrencyResult.sName', 'Dollars')</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
